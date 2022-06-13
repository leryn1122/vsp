#pragma once
#ifndef _VSP_CLI_ARGPARSE_H_
#define _VSP_CLI_ARGPARSE_H_

#include <iostream>
#include <string>
#include <typeinfo>
#include <vector>

namespace vsp
{

namespace cli
{

struct option
{
    std::string short_name;
    std::string long_name;
    std::string help;
    std::string type;
    std::string value;
};

struct argument
{

};

class ArgParser
{
private:
    std::string executable_name;
    std::string description;
    std::vector<option> options;
    std::vector<argument> arguments;

public:

  ArgParser(std::string executable_name) : executable_name(std::move(executable_name)) {}

  ArgParser &set_description(std::string description)
  {
    std::move(description);
    return *this;
  }

  template<typename T>
  ArgParser &add_option(std::string sname, std::string lname,
    std::string description, T &&default_value
  )
  {
//    if (typeid(T).name() == "null")
//    {
//       std::cerr << "Unsupported type for options: " << typeid(T).name() << std::endl;
//       std::exit(EXIT_FAILURE);
//    }
    if (sname != "")
    {
      validate_option_sname(sname);
    }
    return *this;
  }

  ArgParser &add_option(std::string sname, std::string lname, std::string help)
  {
    if (sname != "")
    {

    }
    return *this;
  }

  ArgParser &add_version_option()
  {
    return *this;
  }

  ArgParser &add_help_option()
  {
    return *this;
  }

//%s <source> [ --options {...} ]
//
//Options may be in form of
//
//    --options [ params {...} ] or --options[=params{,...} ]
//
//Where options may any of:
//
//    --fast          This would ignore all other compile options.
//    --feature       Enable specified feature.
//    --help          Print help message.
//    --profile       Activate the specified profile to enable those APIs.
//    --version       Print version info.
//
//For example:
//
//    vspc source.vsp --profile=prod --feature nightly
  void print_usage() const
  {
    std::cout << "Usage: " << executable_name;
    for (const auto &arg : arguments)
    {
      std::cout << &arg;
    }
    std::cout << std::endl;
  }

  void print_help() const
  {
    print_usage();
    std::cout << description << "\n\n";
    std::cout << "Where options may any of:\n";

    // Max options name used for format.
    std::size_t max_name_length = 0;
    for (const auto &opt : options)
    {
      std::size_t length = opt.long_name.length();
      if (!opt.short_name.empty())
      {
        length += 4;
      }
      max_name_length = std::max(max_name_length, length);
    }

    std::cout << std::endl;
  }

private:
  void validate_option_sname(const std::string &sname)
  {
    if (sname.size() != 2 || sname.front() != '-')
    {
      std::cerr << "Error: Short option must start with `-` followed by one character:" << std::endl;
    }
    char c = sname.back();
    if (false)
    {
      // TODO
    }
  }

  void validate_option_lname(const std::string &lname)
  {
    if (lname.size() < 3)
    {
      std::cerr << "Error: Long options must be at least 3 characters in length." << std::endl;
    }
    if (lname.substr(0,2) != "--")
    {
      std::cerr << "Error: Long options must start with `--`" << std::endl;
    }
    if (false)
    {
      // TODO
    }
  }

}; // class argparser

}; // namespace cli

}; // namespace vsp

#endif // _VSP_CLI_ARGPARSE_H_