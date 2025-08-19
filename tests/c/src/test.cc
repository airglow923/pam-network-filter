#include "gtest/gtest.h"

#include "pam.h"

extern struct pam_module _pam_listfile_modstruct;

namespace {

TEST(PamListfileModstruct, name) {
  EXPECT_STREQ(_pam_listfile_modstruct.name, "pam_network_filter");
}

TEST(PamListfileModstruct, pam_sm_authenticate) {
  EXPECT_NE(_pam_listfile_modstruct.pam_sm_authenticate, nullptr);
}

TEST(PamListfileModstruct, pam_sm_setcred) {
  EXPECT_NE(_pam_listfile_modstruct.pam_sm_setcred, nullptr);
}

TEST(PamListfileModstruct, pam_sm_acct_mgmt) {
  EXPECT_EQ(_pam_listfile_modstruct.pam_sm_acct_mgmt, nullptr);
}

TEST(PamListfileModstruct, pam_sm_open_session) {
  EXPECT_EQ(_pam_listfile_modstruct.pam_sm_open_session, nullptr);
}

TEST(PamListfileModstruct, pam_sm_close_session) {
  EXPECT_EQ(_pam_listfile_modstruct.pam_sm_close_session, nullptr);
}

TEST(PamListfileModstruct, pam_sm_chauthtok) {
  EXPECT_EQ(_pam_listfile_modstruct.pam_sm_chauthtok, nullptr);
}

} // namespace
