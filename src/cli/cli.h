#pragma once
#ifndef _VSP_CLI_H_
#define _VSP_CLI_H_

namespace vsp
{

namespace cli
{

void
do_print_help_and_exit();

void
fast_return(char *argv[], char* cmd, void(*ptr)());

void
fast_return_without_primary_args(char *argv[], char* cmd);

void
do_print_version_and_exit(char* cmd);

} // namespace vsp::cli
} // namespace vsp

#endif // _VSP_CLI_H_