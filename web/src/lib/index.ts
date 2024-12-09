import { createClient } from "@supabase/supabase-js"
import { PUBLIC_SB_KEY, PUBLIC_SB_URL } from "$env/static/public"

export const supabase = createClient(PUBLIC_SB_URL, PUBLIC_SB_KEY)
