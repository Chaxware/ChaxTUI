#include <string>
#include <future>

#include <Hub.hpp>
#include <NetworkManager.hpp>

#include <FtxUI.h>


int main() {
  using namespace ftxui;

  NetworkManager net_man;

  std::vector<Hub> user_hubs = {Hub("DMs"), Hub("TestSubject-1"), Hub("TestSubject-2")};

  auto hub_fetch_future = std::async(std::launch::async,
                      [&user_hubs, &net_man] { net_man.fetchHubMetaInto(user_hubs); });


  /*---*--- UI Code---*---*/
  auto screen = ScreenInteractive::Fullscreen();

  // just a text field
  std::string message_typed;  //
  auto        text_field = Input(&message_typed, "Type your message...");


  // create hubs dropdown
  int  selected_hub;
  std::vector<std::string> user_hub_names;

  // push the names of the hub into the vector above
  std::transform(user_hubs.begin(), user_hubs.end(), std::back_inserter(user_hub_names),
                 [](const Hub& h) { return h.Name; });

  auto hub_dropdown = Dropdown(&user_hub_names, &selected_hub);


  // fetch the channels of the hub
  std::vector<Channel> hub_channels;
  auto channel_fetch_future = std::async(std::launch::async, [&hub_channels, &user_hubs, selected_hub] {
      user_hubs[selected_hub].fetchChannelsInto(hub_channels);
  });
}
