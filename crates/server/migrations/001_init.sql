-- ============================================
-- 用户核心表
-- ============================================
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(64) UNIQUE,
    email VARCHAR(255) UNIQUE,
    email_verified_at TIMESTAMPTZ,
    display_name VARCHAR(128),
    avatar_url TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_status ON users(status);

-- ============================================
-- 认证方式 1: 密码登录
-- ============================================
CREATE TABLE user_credentials (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    password_hash VARCHAR(255) NOT NULL,
    password_changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    failed_attempts INT NOT NULL DEFAULT 0,
    locked_until TIMESTAMPTZ
);

-- ============================================
-- 认证方式 2: OAuth
-- ============================================
CREATE TABLE oauth_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(32) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    token_expires_at TIMESTAMPTZ,
    raw_data JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(provider, provider_user_id)
);

CREATE INDEX idx_oauth_user ON oauth_accounts(user_id);

-- ============================================
-- 认证方式 3: Passkey (WebAuthn)
-- ============================================
CREATE TABLE passkeys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    credential_id BYTEA UNIQUE NOT NULL,
    public_key BYTEA NOT NULL,
    sign_count BIGINT NOT NULL DEFAULT 0,
    device_name VARCHAR(128),
    transports TEXT[],
    aaguid BYTEA,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ
);

CREATE INDEX idx_passkey_user ON passkeys(user_id);
CREATE INDEX idx_passkey_credential ON passkeys(credential_id);

-- ============================================
-- 身份组（用户通过组获取权限）
-- ============================================
CREATE TABLE groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(64) UNIQUE NOT NULL,
    name VARCHAR(128) NOT NULL,
    description TEXT,
    parent_id UUID REFERENCES groups(id) ON DELETE SET NULL,
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_groups_parent ON groups(parent_id);

-- ============================================
-- 用户-身份组关联
-- ============================================
CREATE TABLE group_members (
    group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (group_id, user_id)
);

CREATE INDEX idx_group_members_user ON group_members(user_id);

-- ============================================
-- 身份组权限（直接存储权限字符串，支持通配符）
-- ============================================
CREATE TABLE group_permissions (
    group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    permission VARCHAR(128) NOT NULL,
    PRIMARY KEY (group_id, permission)
);

-- ============================================
-- 审计日志
-- ============================================
CREATE TABLE auth_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(32) NOT NULL,
    auth_method VARCHAR(32),
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_auth_logs_user ON auth_logs(user_id, created_at DESC);
CREATE INDEX idx_auth_logs_action ON auth_logs(action, created_at DESC);

-- ============================================
-- 初始数据: 系统身份组
-- ============================================
INSERT INTO groups (code, name, description, is_system) VALUES
    ('admin', '管理员', '系统管理员，拥有所有权限', TRUE),
    ('user', '普通用户', '默认用户身份组', TRUE);

-- ============================================
-- 初始数据: 身份组权限
-- ============================================
-- 管理员拥有所有权限
INSERT INTO group_permissions (group_id, permission)
SELECT id, '*' FROM groups WHERE code = 'admin';

-- 普通用户基础权限
INSERT INTO group_permissions (group_id, permission)
SELECT id, unnest(ARRAY[
    'chat.send',
    'chat.read',
    'model.gpt-3.5-turbo.use',
    'conversation.create',
    'conversation.read',
    'conversation.delete'
]) FROM groups WHERE code = 'user';
