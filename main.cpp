#include <string>
#include <future>

#include <Hub.hpp>
#include <NetworkManager.hpp>

#include <FtxUI.h>

int main()
{

  using namespace ftxui;

  NetworkManager net_man;
  Hub dms = Hub("DMs");
  Hub test_subject_1 = Hub("TestSubject-1");

  Channel DM_1;
  Channel DM_2;
  DM_1.Id = 1440;
  DM_2.Id = 1441;
  DM_1.Name = "DM_1";
  DM_2.Name = "DM_2";
  DM_1.messages.push_back("HI there, I am DM-1");
  DM_2.messages.push_back("HI there, I am DM-2");
  DM_1.messages.push_back("This is the second message in DM-1");
  DM_2.messages.push_back("This is the ninth message in DM-2");

  Channel test_subject_1_ch1;
  Channel test_subject_1_ch2;
  test_subject_1_ch1.Id = 1442;
  test_subject_1_ch2.Id = 1443;
  test_subject_1_ch1.Name = "test_subject_1_ch1";
  test_subject_1_ch2.Name = "test_subject_1_ch2";
  test_subject_1_ch1.messages.push_back("HI there, I am test_subject_1_ch1");
  test_subject_1_ch2.messages.push_back("HI there, I am test_subject_1_ch2");
  test_subject_1_ch1.messages.push_back("Hamza sucks - test_subject_1_ch1");
  test_subject_1_ch2.messages.push_back("avob is a cutie ngl");

  dms.Channels.push_back(DM_1);
  dms.Channels.push_back(DM_2);
  test_subject_1.Channels.push_back(test_subject_1_ch1);
  test_subject_1.Channels.push_back(test_subject_1_ch2);

  // dms.Channels.push_back()
  std::vector<Hub> user_hubs = {dms, test_subject_1};

  auto hub_fetch_future = std::async(std::launch::async,
                                     [&user_hubs, &net_man]
                                     { net_man.fetchHubMetaInto(user_hubs); });

  /*---*--- UI Code---*---*/

  // just a text field
  std::string message_typed; //
  auto text_field = Input(&message_typed, "Type your message...");

  // create hubs dropdown
  int selected_hub = 0;
  std::vector<std::string> user_hub_names;

  // push the names of the hub into the vector above
  std::transform(user_hubs.begin(), user_hubs.end(), std::back_inserter(user_hub_names),
                 [](const Hub &h)
                 { return h.Name; });

  auto hub_dropdown = Dropdown(&user_hub_names, &selected_hub);

  // fetch the channels of the hub
  std::vector<Channel> hub_channels = user_hubs[selected_hub].Channels;
  // auto channel_fetch_future = std::async(std::launch::async, [&hub_channels, &user_hubs, selected_hub]
  //                                        { user_hubs[selected_hub].fetchChannelsInto(hub_channels); });

  // create channels dropdown
  int selected_channel = 0;
  std::vector<std::string> hub_channel_names;
  for (auto c : hub_channels)
  {
    hub_channel_names.push_back(c.Name); // populate channel names
  }
  auto channel_dropdown = Dropdown(&hub_channel_names, &selected_channel);

  // create text area for messages
  std::vector<Element> text_area_components = {};
  Element text_area = vbox(text_area_components);

  auto channel_msgs = user_hubs[selected_hub].Channels[selected_channel].messages;
  for (std::string m : channel_msgs)
  {
    text_area_components.push_back(text(m)); // populate texts from texts in channel
  }
  // Catch events
  int msg_offset = 0;
  text_field |= CatchEvent([&](auto event)
                           {
    bool ret = Event::Character('\n') == event;
    if (ret){
      if (message_typed.size() > 0){
        user_hubs[selected_hub].Channels[selected_channel].messages.push_back(message_typed);
        message_typed.clear();
      }
    }
    return ret; });

  text_field |= CatchEvent([&](auto event)
                           {
    bool ret = Event::ArrowUp == event;
    if (ret){
      if (msg_offset < user_hubs[selected_hub].Channels[selected_channel].messages.size()){
        msg_offset += 1;
      }
    }
    return ret; });
  text_field |= CatchEvent([&](auto event)
                           {
    bool ret = Event::ArrowDown == event;
    if (ret){
      if (msg_offset > 0){
        msg_offset -= 1;
      }
    }
    return ret; });

  auto screen = ftxui::ScreenInteractive::Fullscreen();
  auto components = Container::Horizontal({text_field, hub_dropdown, channel_dropdown});
  auto renderer = Renderer(components, [&]
                           {
                             hub_channel_names.clear();
                             for (auto c : user_hubs[selected_hub].Channels)
                             {
                               hub_channel_names.push_back(c.Name);
                             }
                             text_area_components.clear();
                             for (int i = 0; i < Terminal::Size().dimy - 3 - user_hubs[selected_hub].Channels[selected_channel].messages.size(); i++)
                             {
                               text_area_components.push_back(text(" "));
                             }

                             for (int i = 0 + msg_offset; i < user_hubs[selected_hub].Channels[selected_channel].messages.size(); i++)
                             {
                               text_area_components.push_back(text(user_hubs[selected_hub].Channels[selected_channel].messages[i]));
                             }
                             text_area = vbox(text_area_components);
                             return gridbox({
                                 {text_area | ftxui::flex_grow | yframe, hub_dropdown->Render(), channel_dropdown->Render()},
                                 {text_field->Render() | border}
                             }); });
  screen.Loop(renderer);
}