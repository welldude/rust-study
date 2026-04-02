# 完整 Express 实现（与 `src/main.rs` 路由对齐）

下面是一份**可独立运行**的 Node.js + Express + `pg` 实现，对应 `main.rs` 中的路由设计：

- `GET /` — 欢迎文案（纯文本）
- `GET /users` — 列出所有用户（JSON 数组）
- `POST /users` — 创建用户（201 + JSON 单条）
- `GET /users/:id` — 查询单个用户
- `PUT /users/:id` — 更新用户（body：`name`、`email`）
- `DELETE /users/:id` — 删除用户（成功返回 204）

环境与 Rust 版一致：需要 **`DATABASE_URL`**（例如 `postgres://user:password@localhost:5432/simple_api`）。启动时会执行与 `migrations/0001_users_table.sql` 等价的建表语句（`CREATE TABLE IF NOT EXISTS users ...`）。

默认监听 **`0.0.0.0:8000`**，与 Rust 示例相同。

---

## 1. 目录结构（自行新建）

```text
express-app/
  package.json
  index.js
```

把下面两个文件内容分别保存即可。

---

## 2. `package.json`

```json
{
  "name": "user-api-express",
  "version": "1.0.0",
  "private": true,
  "description": "Express mirror of rust-study src/main.rs users API",
  "main": "index.js",
  "scripts": {
    "start": "node index.js"
  },
  "dependencies": {
    "express": "^4.21.2",
    "pg": "^8.14.0"
  }
}
```

---

## 3. `index.js`（完整实现）

```javascript
/**
 * Express + pg — aligns with rust-study src/main.rs routes.
 * Env: DATABASE_URL (required), PORT (optional, default 8000)
 */

const express = require("express");
const { Pool } = require("pg");

const DATABASE_URL = process.env.DATABASE_URL;
if (!DATABASE_URL) {
  console.error("DATABASE_URL must be set");
  process.exit(1);
}

const pool = new Pool({ connectionString: DATABASE_URL });

async function ensureSchema() {
  await pool.query(`
    CREATE TABLE IF NOT EXISTS users (
      id SERIAL PRIMARY KEY,
      name TEXT NOT NULL,
      email TEXT NOT NULL UNIQUE
    );
  `);
}

const app = express();
app.use(express.json());

// GET / — same idea as Axum root()
app.get("/", (_req, res) => {
  res.type("text/plain").send("Welcome to the User Management API!");
});

// GET /users — list_users
app.get("/users", async (_req, res) => {
  try {
    const { rows } = await pool.query(
      "SELECT id, name, email FROM users ORDER BY id",
    );
    res.json(rows);
  } catch {
    res.sendStatus(500);
  }
});

// POST /users — create_user
app.post("/users", async (req, res) => {
  const { name, email } = req.body ?? {};
  if (typeof name !== "string" || typeof email !== "string") {
    return res.sendStatus(400);
  }
  try {
    const { rows } = await pool.query(
      "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
      [name, email],
    );
    res.status(201).json(rows[0]);
  } catch {
    res.sendStatus(500);
  }
});

// GET /users/:id — get_user
app.get("/users/:id", async (req, res) => {
  const id = Number(req.params.id);
  if (!Number.isInteger(id)) return res.sendStatus(400);
  try {
    const { rows } = await pool.query(
      "SELECT id, name, email FROM users WHERE id = $1",
      [id],
    );
    if (rows.length === 0) return res.sendStatus(404);
    res.json(rows[0]);
  } catch {
    res.sendStatus(500);
  }
});

// PUT /users/:id — update_user
app.put("/users/:id", async (req, res) => {
  const id = Number(req.params.id);
  if (!Number.isInteger(id)) return res.sendStatus(400);
  const { name, email } = req.body ?? {};
  if (typeof name !== "string" || typeof email !== "string") {
    return res.sendStatus(400);
  }
  try {
    const { rows } = await pool.query(
      "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email",
      [name, email, id],
    );
    if (rows.length === 0) return res.sendStatus(404);
    res.json(rows[0]);
  } catch {
    res.sendStatus(500);
  }
});

// DELETE /users/:id — delete_user
app.delete("/users/:id", async (req, res) => {
  const id = Number(req.params.id);
  if (!Number.isInteger(id)) return res.sendStatus(400);
  try {
    const { rowCount } = await pool.query("DELETE FROM users WHERE id = $1", [
      id,
    ]);
    if (rowCount === 0) return res.sendStatus(404);
    res.sendStatus(204);
  } catch {
    res.sendStatus(500);
  }
});

const port = Number(process.env.PORT) || 8000;

ensureSchema()
  .then(() => {
    app.listen(port, "0.0.0.0", () => {
      console.log(`Server running on port ${port}`);
    });
  })
  .catch((err) => {
    console.error("Failed to init DB:", err);
    process.exit(1);
  });
```

---

## 4. 安装与运行

在 **`express-app`** 目录下执行：

```bash
npm install
```

Windows PowerShell 示例：

```powershell
$env:DATABASE_URL = "postgres://user:password@localhost:5432/simple_api"
npm start
```

可选端口：

```powershell
$env:PORT = "3000"
npm start
```

---

## 5. 与 Rust 行为差异（要点）

| 点 | Rust（当前仓库） | 本 Express |
|----|------------------|------------|
| `/users` GET/POST | 已实现 | 已实现 |
| `/users/:id` | `main` 已注册，Rust 文件里 handler 未写全时会编译失败 | 已实现 GET/PUT/DELETE |
| 请求体验证 | serde 缺字段时多为 422/400（取决于配置） | POST/PUT 对 `name`/`email` 做了简单 400 |
| 迁移 | `sqlx::migrate!()` + `migrations/` | 启动 `ensureSchema()` 内联 SQL |

---

## 6. 快速自测（可选）

```bash
curl -s http://127.0.0.1:8000/
curl -s http://127.0.0.1:8000/users
curl -s -X POST http://127.0.0.1:8000/users -H "Content-Type: application/json" -d "{\"name\":\"a\",\"email\":\"a@x.com\"}"
```

以上为单一 Markdown 文件内嵌的**完整 Express 工程内容**；复制出 `package.json` 与 `index.js` 即可独立运行。
