#pragma once
#ifndef _VSP_CLI_ARGPARSE_H_
#define _VSP_CLI_ARGPARSE_H_

#include <functional>
#include <iostream>
#include <string>
#include <typeinfo>
#include <vector>

namespace vsp
{

namespace cli
{

namespace
{

template <typename T>
inline std::string type_string()
{
  return "null";
}

template <>
inline std::string type_string<bool>()
{
  return "bool";
}

template <>
inline std::string type_string<int>()
{
  return "int";
}

template <>
inline std::string type_string<int64_t>()
{
  return "int64_t";
}

template <>
inline std::string type_string<double>()
{
  return "double";
}

template <>
inline std::string type_string<std::string>()
{
  return "string";
}

} // namespace

struct option
{
    option(std::string sname, std::string lname, std::string help, std::string type, std::string value)
        : short_name(std::move(sname))
        , long_name(std::move(lname))
        , help(std::move(help))
        , type(std::move(type))
        , value(std::move(value))
    {}

    std::string short_name;
    std::string long_name;
    std::string help;
    std::string type;
    std::string value;
};

struct short_circuit_option
{
    short_circuit_option(std::string sname, std::string lname, std::string help, std::function<void(void)> callback)
        : short_name(std::move(sname))
        , long_name(std::move(lname))
        , help(std::move(help))
        , callback(std::move(callback))
    {}

    std::string short_name;
    std::string long_name;
    std::string help;
    std::function<void(void)> callback;
};

struct argument
{

};

class ArgParser
{
private:
    std::string executable;
    std::string intro;
    std::vector<option> options;
    std::vector<short_circuit_option> short_circuit_options;
    std::vector<argument> arguments;
    std::string example;

public:

  ArgParser(std::string executable) : executable(std::move(executable)) {}

  ArgParser &set_intro(std::string intro)
  {
    this->intro = std::move(intro);
    return *this;
  }

  ArgParser &set_example(std::string example)
  {
    this->example = std::move(example);
    return *this;
  }

  template<typename T>
  ArgParser &add_option(std::string sname, std::string lname,
    std::string description, T &&default_value
  )
  {
    if (typeid(T).name() == "null")
    {
      std::cerr << "Unsupported type for options: " << typeid(T).name() << std::endl;
      std::exit(EXIT_FAILURE);
    }
    if (sname != "")
    {
      validate_option_sname(sname);
    }
    validate_option_lname(lname);
    return *this;
  }

  ArgParser &add_option(std::string sname, std::string lname, std::string help)
  {
    if (sname != "")
    {
      validate_option_sname(sname);
    }
    validate_option_lname(lname);
    options.emplace_back(
      std::move(sname), std::move(lname), std::move(help), "bool", "0");
    return *this;
  }

  /**
   *
   */
  ArgParser &add_short_circuit(std::string sname, std::string lname, std::string help, std::function<void(void)> callback)
  {
    if (sname != "")
    {
      validate_option_sname(sname);
    }
    validate_option_lname(lname);
    short_circuit_options.emplace_back(
      std::move(sname), std::move(lname), std::move(help), std::move(callback));
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

  void print_help() const
  {
    // Print usage:
    std::cout << intro << "\n\n";
    std::cout << "Usage: " << executable << "\n\n";
    for (const auto &arg : arguments)
    {
      std::cout << &arg;
    }

    std::cout <<
//==============================================================================
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""" R"(
Options may be in form of:

    --options [ params {...} ] or --options[=params{,...} ]

Where options may any of:

)" """"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
//==============================================================================
    ;

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
    max_name_length = std::max(max_name_length, std::size_t(21));

    for (const auto &opt : options)
    {
      std::cout << "    ";
      std::size_t printed_length = 0;
      if (!opt.short_name.empty())
      {
        std::cout << opt.short_name << ", ";
      }
      else
      {
        std::cout << "    ";
      }
      printed_length = 4;
      std::cout << opt.long_name;
      printed_length += opt.long_name.length();
      std::cout << std::string(max_name_length - printed_length, ' ');
      if (opt.type != "bool")
      {
          std::cout << "(" << opt.type << ") ";
      }
      std::cout << opt.help << '\n';
    }

    // Print Example
    std::cout << "\nFor example:\n";
    std::cout << example;
    std::cout << std::endl;
  }

  void parse(int argc, char *argv[])
  {
     print_help();
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