rootProject.name = "vsp"

buildCache {
  local {
    isEnabled = System.getenv("CI") == null
    directory = File(rootDir, "build/build-cache")
    removeUnusedEntriesAfterDays = 7
  }
}
