#include <vector>
#include <Hub.hpp>

#pragma once


class NetworkManager {
public:
    NetworkManager() = default;

    // fetch all the Hubs the user is in and append them to the vector
    void fetchHubMetaInto(std::vector<Hub>&);

    // fetch the names of all Hubs into the vector
    void fetchHubNamesInto(std::vector<std::string>&);
};
