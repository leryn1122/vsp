#pragma once
#ifndef _VSP_CLI_ARGPARSE_H_
#define _VSP_CLI_ARGPARSE_H_

#include <functional>
#include <iostream>
#include <sstream>
#include <string>
#include <typeinfo>
#include <vector>

#include "version.hpp"

/**
 *  Reference on: https://github.com/0382/util/cpp/argparse/argparse.hpp
 *  
 *  Create an argument parser instance in chain invocation of option registrations.
 *  
 *  Options are seperated into two types:
 *    - Normal option:
 *       
 *    - Short circuit option
 *      Once met short circuit, the program would invoke callback, and be 
 *      exited immediately.
 *  
 *  
 *  
 *  
 *  ```cpp
 *  ArgParser arg_parser = vsp::cli::ArgParser(CMD)
 *      .set_intro("Vsp Language Compiler")
 *      .add_help_option()
 *      .add_version_option()
 *      .add_option("-d", "--debug", "Enable debug mode.")
 *      .add_option("", "--feature", "Enable specified feature.")
 *      .add_option("", "--profile", "Activate the specified profile to enable those APIs.")
 *      .add_option<int>("-t", "--thread", "Set the parallel thread to compile source.", 4)
 *      .add_option("-v", "--verbose", "Enable verbose mode.")
 *      .set_example("");
 *      .parse(argc, argv);
 *  ```
 */
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

template <typename T>
std::string to_string(const T &value)
{
  std::ostringstream oss;
  oss << value;
  return oss.str();
}

};  /*--  namespace  --*/

struct option
{
  option(std::string short_name, std::string long_name, std::string description, std::string type, std::string value)
    : short_name(std::move(short_name))
    , long_name(std::move(long_name))
    , description(std::move(description))
    , type(std::move(type))
    , value(std::move(value))
  {}

  std::string short_name;
  std::string long_name;
  std::string description;
  std::string type;
  std::string value;
};  /*--  struct option  --*/

struct short_circuit_option
{
  short_circuit_option(std::string short_name, std::string long_name, std::string description, std::function<void(void)> callback)
    : short_name(std::move(short_name))
    , long_name(std::move(long_name))
    , description(std::move(description))
    , callback(std::move(callback))
  {}

  std::string short_name;
  std::string long_name;
  std::string description;
  std::function<void(void)> callback;
};  /*--  struct short_circuit_option  --*/

struct argument
{
  argument(std::string name, std::string description)
    : name(std::move(name))
    , description(std::move(description))
  {}

  std::string name;
  std::string description;
  std::string type;
  std::string value;
};  /*--  struct argument  --*/

class ArgParser
{
private:
  std::string executable;
  std::string intro;
  std::vector<argument> arguments;
  std::vector<option> options;
  std::vector<short_circuit_option> short_circuit_options;
  std::string example;

public:

  ArgParser() {}
  ArgParser(std::string executable) : executable(std::move(executable)) {}
  ~ArgParser() {}

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

  ArgParser &add_argument(std::string name, std::string description)
  {
    this->arguments.emplace_back(
      std::move(name), std::move(description));
    return *this;
  }
  
  template<typename T>
  ArgParser &add_option(std::string short_name, std::string long_name,
                        std::string description, T &&default_value)
  {
    if (type_string<T>() == "null")
    {
      std::cerr << "Error: Unsupported type for options: " << typeid(T).name() << std::endl;
      std::exit(EXIT_FAILURE);
    }
    if (short_name != "")
    {
      validate_option_short_name(short_name);
    }
    validate_option_long_name(long_name);
    options.emplace_back(
      std::move(short_name), std::move(long_name), std::move(description), type_string<T>(), to_string(default_value));
    return *this;
  }

  ArgParser &add_option(std::string short_name, std::string long_name, std::string description)
  {
    if (short_name != "")
    {
      validate_option_short_name(short_name);
    }
    validate_option_long_name(long_name);
    options.emplace_back(
      std::move(short_name), std::move(long_name), std::move(description), "bool", "0");
    return *this;
  }

  ArgParser &add_short_circuit_option(std::string short_name, std::string long_name,
                                      std::string help, std::function<void(void)> callback)
  {
    if (short_name != "")
    {
      validate_option_short_name(short_name);
    }
    validate_option_long_name(long_name);
    short_circuit_options.emplace_back(
      std::move(short_name), std::move(long_name), std::move(help), std::move(callback));
    return *this;
  }

  ArgParser &add_help_option()
  {
    return add_short_circuit_option("-h", "--help", "Print help info.",
      [this]() {
        this->print_help();
      });
  }

  ArgParser &add_version_option()
  {
    return add_short_circuit_option("-v", "--version", "Print version info.",
      [this]() {
        std::cout << executable << " version " << vsp_VERSION << std::endl;
      });
  }

  void print_help()
  {
    // Print usage:
    std::cout << intro << "\n";
    std::cout << "\nUsage: " << executable << "\n\n";
    for (const auto &arg : arguments)
    {
      std::cout << "    <" << arg.name << ">    " << arg.description;
    }

    std::cout <<
//==============================================================================
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""" R"(
Options may be in form of:

    --options [ params {...} ]

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
    max_name_length = std::max(max_name_length, std::size_t(25));

    for (const auto &opt : options)
    {
      std::cout << "    ";
      std::size_t printed_length = 4;
      if (!opt.short_name.empty())
      {
        std::cout << opt.short_name << ", ";
      }
      else
      {
        std::cout << "    ";
      }
      std::cout << opt.long_name;
      printed_length += opt.long_name.length();
      std::cout << std::string(max_name_length - printed_length, ' ');
      std::cout << opt.description << '\n';
    }

    // Print Example
    if (this->example.size() > 0)
    {
      std::cout << "\nFor example:\n";
      std::cout << example;
      std::cout << std::endl;
    }

  }

  /**
   * Entry point to parse arguments after option registration.
   */
  ArgParser parse(int argc, char *argv[])
  {
    // Fast return if no arguments accepted. 
    if (argc == 1 && arguments.size() > 0)
    {
      print_help();
      std::exit(EXIT_SUCCESS);
    }

    std::vector<std::string> tokens;
    for (int i = 1; i < argc; i++)
    {
      tokens.emplace_back(argv[i]);
    }

    // Fast return if met short circuit option.
    for (auto &&short_circuit_option : this->short_circuit_options)
    {
      auto pos = std::find_if(tokens.cbegin(), tokens.cend(),
        [&short_circuit_option](const std::string &token) {
          return token == short_circuit_option.short_name
              || token == short_circuit_option.long_name;
        });
      if (pos != tokens.cend())
      {
        short_circuit_option.callback();
        std::exit(EXIT_SUCCESS);
      }
    }

    // Iterate on this->options.
    for (auto &&option : this->options)
    {
      auto pos = std::find_if(tokens.cbegin(), tokens.cend(),
        [&option](const std::string &token) {
          return token == option.short_name || token == option.long_name;
        }
      );

      // If the option is any one of tokens, jump to the next loop.
      if (pos == tokens.cend())
      {
        continue;
      }

      // Remove the token if matched.
      pos = tokens.erase(pos);
      if (option.type == "bool")
      {
        option.value = "1";
      }
      // Other types need to consume following tokens
      else
      {
        if (pos == tokens.cend())
        {
          std::cerr << "Error: Option `" << option.short_name << "`, `" << option.long_name
                    << "` does not have enough arguments." << std::endl;
          std::exit(EXIT_FAILURE);
        }
        for( ; pos != tokens.cend() ; pos++)
        {
          if ((*pos).front() != '-')
          {
            option.value = *pos;
            pos = tokens.erase(pos);
          }
          else
          {
            break;
          }
        }
      }
    }

    // Parse arguments.
    // Check whether the rest tokens equal to needed in amount.
    if (tokens.size() != arguments.size())
    {
      std::cerr << "Error: Does not have enough arguments." << std::endl;
      std::exit(EXIT_FAILURE);
    }
    // Iteratively consume the rest arguments.
    for (auto &arg : arguments)
    {
      for (auto pos = tokens.begin(); pos != tokens.end();)
      {
        if (try_parse_argument(*pos, arg))
        {
          arg.value = *pos;
          pos = tokens.erase(pos);
          break;
        }
        pos++;
      }
      if (arg.value == "")
      {
        std::cerr << "Error: Argument `" << arg.name << "` should have value." << std::endl;
        std::exit(EXIT_FAILURE);
      }
    }

    //for (std::size_t i = 0; i < tokens.size(); i++)
    //{
    //  arguments[i].value = tokens[i];
    //}

    return *this;
  }

  bool has_option(std::string name)
  {
    return std::any_of(options.cbegin(), options.cend(),
        [&name](const option &option) { return option.long_name == name; });
  }

private:
  bool try_parse_argument(const std::string &line, argument &arg)
  {
    if ((line).substr(0,2) == "--")
    {
      std::cerr << "Error: Invalid options :"  << line << std::endl;
      std::exit(EXIT_FAILURE);
    }
    return true;
  }

  using short_circuit_option_iterator = std::vector<short_circuit_option>::const_iterator;
  using option_iterator = std::vector<option>::const_iterator;
  using argument_iterator = std::vector<argument>::const_iterator;

  // Find option.

  auto find_short_circuit_option_short_name(const std::string &name) const -> short_circuit_option_iterator
  {
    return std::find_if(short_circuit_options.cbegin(), short_circuit_options.cend(),
        [&name](const short_circuit_option &option) { return option.short_name == name; });
  }

  auto find_short_circuit_option_long_name(const std::string &name) const -> short_circuit_option_iterator
  {
    return std::find_if(short_circuit_options.cbegin(), short_circuit_options.cend(),
        [&name](const short_circuit_option &option) { return option.long_name == name; });
  }

  auto find_option_short_name(const std::string &name) const -> option_iterator
  {
    return std::find_if(options.cbegin(), options.cend(),
        [&name](const option &option) { return option.short_name == name; });
  }

  auto find_option_long_name(const std::string &name) const -> option_iterator
  {
    return std::find_if(options.cbegin(), options.cend(),
        [&name](const option &option) { return option.long_name == name; });
  }

  // Check option.

  void validate_option_short_name(const std::string &short_name)
  {
    if (short_name.size() != 2 || short_name.front() != '-')
    {
      std::cerr << "Error: Short option must start with `-` followed by one character:" << std::endl;
      std::exit(EXIT_FAILURE);
    }
    if (find_option_long_name(short_name) != options.cend())
    {
      std::cerr << "Error: Short option " << short_name << " has already existed." << std::endl;
      std::exit(EXIT_FAILURE);
    }
  }

  void validate_option_long_name(const std::string &long_name)
  {
    if (long_name.size() < 3)
    {
      std::cerr << "Error: Long options must be at least 3 characters in length." << std::endl;
      std::exit(EXIT_FAILURE);
    }
    if (long_name.substr(0,2) != "--")
    {
      std::cerr << "Error: Long options must start with `--`." << std::endl;
      std::exit(EXIT_FAILURE);
    }
    if (find_option_long_name(long_name) != options.cend()
        || find_short_circuit_option_long_name(long_name) != short_circuit_options.cend())
    {
      std::cerr << "Error: Long option " << long_name << " has already existed." << std::endl;
      std::exit(EXIT_FAILURE);
    }
  }

};  /*--  class argparser  --*/

};  /*--  namespace vsp::cli  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_CLI_ARGPARSE_H_  --*/