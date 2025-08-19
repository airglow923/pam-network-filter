#ifdef __cplusplus
extern "C" {
#endif

#include <security/pam_modules.h>

// https://github.com/linux-pam/linux-pam/blob/2f42cd380c92907ed8d6a45fc6cada3df900b4c2/libpam/include/security/pam_modules.h#L37-L54
struct pam_module {
  const char *name;
  int (*pam_sm_authenticate)(pam_handle_t *pamh, int flags, int argc,
                             const char **argv);
  int (*pam_sm_setcred)(pam_handle_t *pamh, int flags, int argc,
                        const char **argv);
  int (*pam_sm_acct_mgmt)(pam_handle_t *pamh, int flags, int argc,
                          const char **argv);
  int (*pam_sm_open_session)(pam_handle_t *pamh, int flags, int argc,
                             const char **argv);
  int (*pam_sm_close_session)(pam_handle_t *pamh, int flags, int argc,
                              const char **argv);
  int (*pam_sm_chauthtok)(pam_handle_t *pamh, int flags, int argc,
                          const char **argv);
};

#ifdef __cplusplus
}
#endif
