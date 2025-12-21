; NSIS Installer Hooks for InterviewSpark
; Removes user data on uninstall

!macro NSIS_HOOK_PREUNINSTALL
  ; Remove user data directory on uninstall
  RMDir /r "$APPDATA\com.interviewspark.app"
!macroend
