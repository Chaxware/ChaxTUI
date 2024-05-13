#include <Hub.hpp>

void Hub::fetchChannelsInto(std::vector<Channel> &)
{
  // TODO
}

Hub::Hub(const std::string &s_name)
{
  Name = s_name;
}

void Hub::fetchChannelNamesInto(std::vector<std::string> &)
{
  // TODO
}

std::string Channel::fetchChannelDescription()
{
  return std::string();
}
