#include <string>
#include <vector>

#pragma once

// channel metadata
struct Channel
{
    std::string Name;
    int Id;
    std::vector<std::string> messages;
    std::string fetchChannelDescription();
};

class Hub
{
public:
    std::string Name;
    std::vector<Channel> Channels;

    explicit Hub(const std::string &);

    void fetchChannelsInto(std::vector<Channel> &);
    void fetchChannelNamesInto(std::vector<std::string> &);
};
