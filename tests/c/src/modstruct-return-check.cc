#include "gtest/gtest.h"

#include "pam.h"

extern struct pam_module _pam_listfile_modstruct;

namespace {

TEST(PamListfileModstruct, pam_sm_acct_mgmt_return_check) {
  EXPECT_EQ(_pam_listfile_modstruct.pam_sm_acct_mgmt(nullptr, 0, 0, nullptr),
            PAM_IGNORE);
}

TEST(PamListfileModstruct, pam_sm_open_session_return_check) {
  EXPECT_EQ(_pam_listfile_modstruct.pam_sm_open_session(nullptr, 0, 0, nullptr),
            PAM_IGNORE);
}

TEST(PamListfileModstruct, pam_sm_close_session_return_check) {
  EXPECT_EQ(
      _pam_listfile_modstruct.pam_sm_close_session(nullptr, 0, 0, nullptr),
      PAM_IGNORE);
}

TEST(PamListfileModstruct, pam_sm_chauthtok_return_check) {
  EXPECT_EQ(_pam_listfile_modstruct.pam_sm_chauthtok(nullptr, 0, 0, nullptr),
            PAM_IGNORE);
}

} // namespace
