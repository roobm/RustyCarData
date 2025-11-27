# 🗄️ Supabase Setup Guide

This guide will help you set up Supabase for storing OBD-II telemetry data from your app.

## What is Supabase?

Supabase is an open-source Firebase alternative that provides:
- PostgreSQL database (cloud-hosted)
- Auto-generated REST & GraphQL APIs
- Real-time subscriptions
- **Free tier**: 500MB database, 2GB bandwidth/month

---

## Step 1: Create Supabase Account

1. **Go to**: https://supabase.com
2. **Sign up** using:
   - GitHub (recommended - faster)
   - Google
   - Email

---

## Step 2: Create a New Project

1. Click **"New project"** button
2. Fill in project details:
   - **Organization**: Create new or select existing
   - **Project Name**: `car-telemetry-db` (or any name you like)
   - **Database Password**: 
     - Click "Generate password" (recommended)
     - **⚠️ SAVE THIS PASSWORD!** Copy it to a secure location
   - **Region**: Choose closest to your location
     - US: `us-east-1` or `us-west-1`
     - Europe: `eu-west-1` or `eu-central-1`
     - Asia: `ap-southeast-1`
   - **Pricing Plan**: Select **Free** (default)

3. Click **"Create new project"**
4. Wait 2-3 minutes for provisioning ⏳

---

## Step 3: Get Your API Credentials

Once your project is ready (green checkmark):

1. Go to **Settings** (⚙️ gear icon in left sidebar)
2. Click **API** section
3. You'll see two important values:

### Project URL
```
https://xxxxxxxxxxxxx.supabase.co
```
Copy this - it's your `SUPABASE_URL`

### API Keys
You'll see two keys:
- **anon / public** - This is safe for client-side code ✅
- **service_role** - Server-side only, never expose! ❌

Copy the **anon public** key - it's your `SUPABASE_ANON_KEY`

---

## Step 4: Configure Your App

### 4a. Create `.env` File

```bash
cd src-tauri
cp .env.example .env
```

### 4b. Edit `.env` File

Open `src-tauri/.env` and replace the placeholders:

```bash
# Replace with your actual values from Step 3
SUPABASE_URL=https://xxxxxxxxxxxxx.supabase.co
SUPABASE_ANON_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3M...
```

**⚠️ Important**: Never commit `.env` to git! It's already in `.gitignore`.

---

## Step 5: Run Database Migration

1. In your Supabase dashboard, click **SQL Editor** (left sidebar)
2. Click **"New query"**
3. Copy the entire contents of `src-tauri/migrations/001_initial_schema.sql`
4. Paste into the SQL editor
5. Click **"Run"** (or press `Ctrl+Enter`)

You should see:
```
Success. No rows returned
```

This creates 3 tables:
- `sessions` - Driving session metadata
- `telemetry_points` - Individual OBD-II readings
- `session_stats` - Aggregated statistics

---

## Step 6: Verify Tables Created

1. Click **Table Editor** (left sidebar)
2. You should see 3 tables:
   - ✅ sessions
   - ✅ telemetry_points
   - ✅ session_stats

Click on each table to explore the schema.

---

## Step 7: Add Dependencies

The Supabase client code is already in `src-tauri/src/storage/supabase.rs`.

Add the required Rust dependencies:

```bash
cd src-tauri
cargo add postgrest
cargo add dotenv
```

---

## Step 8: Test Connection

Start your app:

```bash
npm run tauri dev
```

The app should:
1. Load environment variables from `.env`
2. Initialize Supabase client (check console for logs)
3. Store telemetry data in both local CSV + Supabase

---

## Troubleshooting

### Issue: "Failed to load .env file"

**Solution**: Make sure `.env` is in `src-tauri/` directory:
```bash
ls src-tauri/.env
```

### Issue: "Invalid API key"

**Solution**: Double-check you copied the **anon public** key, not service_role:
1. Go to Settings > API
2. Copy from "Project API keys" > "anon" / "public"

### Issue: "Connection refused"

**Solution**: Check your SUPABASE_URL format:
- ✅ Correct: `https://abc123.supabase.co`
- ❌ Wrong: `https://app.supabase.com/project/abc123`

### Issue: "Table does not exist"

**Solution**: Run the migration SQL again (Step 5)

---

## Viewing Your Data

### In Supabase Dashboard

1. Click **Table Editor**
2. Select `telemetry_points` table
3. You'll see live data as you drive in the app!

### Sample Queries

```sql
-- View recent sessions
SELECT * FROM sessions 
ORDER BY created_at DESC 
LIMIT 10;

-- View telemetry for a session
SELECT * FROM telemetry_points 
WHERE session_id = 'your-session-uuid'
ORDER BY timestamp ASC;

-- Calculate session stats
SELECT calculate_session_stats('your-session-uuid');
```

---

## Free Tier Limits

Keep an eye on your usage at Settings > Usage:

- **Database**: 500 MB
- **Bandwidth**: 2 GB/month  
- **File Storage**: 1 GB

For this app:
- Each telemetry point ~200 bytes
- 1 hour of driving (~36,000 points) ≈ 7 MB
- You can store ~70 hours of driving data on free tier

---

## Next Steps

Once Supabase is working:

1. ✅ Data is automatically stored in both CSV + Supabase
2. ✅ You can query historical sessions
3. ✅ AI agents can analyze sessions from cloud storage
4. 🔜 Build a session history UI
5. 🔜 Share sessions with others via URL

---

## Security Notes

✅ **DO**:
- Use the anon/public key in client code
- Store credentials in `.env` (not committed)
- Enable Row Level Security (RLS) for multi-user apps

❌ **DON'T**:
- Commit `.env` to git
- Share your service_role key
- Expose database password

---

## Support

- Supabase Docs: https://supabase.com/docs
- Discord: https://discord.supabase.com
- GitHub Issues: https://github.com/supabase/supabase/issues

Happy storing! 🚗💨
