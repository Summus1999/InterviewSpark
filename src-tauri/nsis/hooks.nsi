; NSIS Installer Hooks for InterviewSpark
; Smart uninstall: preserve data on upgrade, prompt on manual uninstall

!macro NSIS_HOOK_PREUNINSTALL
  ; Check if uninstall is triggered by installer (upgrade scenario)
  ; Silent mode = upgrade from installer, keep user data
  ${If} ${Silent}
    ; Upgrade scenario: skip data removal to preserve user data
    Goto SkipDataRemoval
  ${EndIf}
  
  ; Manual uninstall: ask user whether to keep data
  MessageBox MB_YESNO|MB_ICONQUESTION \
    "是否删除所有用户数据（面试记录、题库等）？$\n$\n选择「否」将保留数据，下次安装可继续使用。" \
    IDNO SkipDataRemoval
  
  ; User chose to delete data
  DetailPrint "Removing user data from AppData..."
  RMDir /r "$APPDATA\com.interviewspark.app"
  
  SkipDataRemoval:
    DetailPrint "User data preserved."
!macroend
