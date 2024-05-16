#include <string>
#include <future>

#include <Hub.hpp>
#include <NetworkManager.hpp>

#include <FtxUI.h>

<<<<<<< Updated upstream
int main()
{

  using namespace ftxui;
=======
using namespace ftxui;

Component Profile_Content()
{
  class Impl : public ComponentBase
  {
  private:
    bool checked[3] = {false, false, false};
    float slider = 50;

  public:
    Impl()
    {
      Add(Container::Vertical({

      }));
    }
  };
  return Make<Impl>();
}

int main()
{
  auto screen = ftxui::ScreenInteractive::Fullscreen();
>>>>>>> Stashed changes

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
<<<<<<< Updated upstream
=======
  DM_1.messages.push_back("Known issues: Focus problems, you need to click on the menu to <focus> on it before the keyboard shortcuts start working");
>>>>>>> Stashed changes
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
<<<<<<< Updated upstream
=======

  // create a menu dropdown, contains Join server, Leave current server, Open profile, etc
  std::vector<std::string> menubutton_entries = {"Join Server", "Leave Server", "My Profile", "Press <TAB> to exit", "Press <ENTER> to continue"};
  int menubutton_selected = 0;
  auto menubutton_menu = Menu({.entries = &menubutton_entries, .selected = &menubutton_selected});
  bool menubutton_open = false;

  // Profile menu
  // Element dummyprofile = text("My Profile");
  bool profile_menu_open = false;
  auto profile_window = Window({
      .inner = Profile_Content(),
  });
  profile_window |= CatchEvent([&](auto event)
                               {
    bool ret = Event::Escape == event;
    if (ret) {
      menubutton_open = false;
      profile_menu_open = false;
    }
    return ret; });
>>>>>>> Stashed changes
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
<<<<<<< Updated upstream

  auto screen = ftxui::ScreenInteractive::Fullscreen();
  auto components = Container::Horizontal({text_field, hub_dropdown, channel_dropdown});
  auto renderer = Renderer(components, [&]
                           {
=======
  hub_dropdown |= CatchEvent([&](auto event)
                             {
                            bool ret = Event::Special({9}) == event;
                            if (ret && menubutton_open == false){
                              menubutton_open = true;
                            } else if (ret && menubutton_open == true){
                              menubutton_open = false;
                            }
                            return ret; });
  channel_dropdown |= CatchEvent([&](auto event)
                                 {
                            bool ret = Event::Special({9}) == event;
                            if (ret && menubutton_open == false){
                              menubutton_open = true;
                            } else if (ret && menubutton_open == true){
                              menubutton_open = false;
                            }
                            return ret; });
  text_field |= CatchEvent([&](auto event)
                           {
                            bool ret = Event::Special({9}) == event;
                            if (ret && menubutton_open == false){
                              menubutton_open = true;
                            } else if (ret && menubutton_open == true){
                              menubutton_open = false;
                            }
                            return ret; });

  menubutton_menu |= CatchEvent([&](auto event)
                                {
                                  bool ret = Event::Special({9}) == event;
                                  if (ret && menubutton_open == true)
                                  {
                                    profile_menu_open = false;
                                    menubutton_open = false;
                                  }
                                  return ret; });
  menubutton_menu |= CatchEvent([&](auto event)
                                {
                            bool ret = Event::Return == event;
                            if (ret){
                              if (menubutton_selected == 0){
                                // its join server, handle join server using netman
                                
                                // join_server_menu = true;
                                // // display the join server menu, handle a "Enter" command and then proceed with following code
                                
                                // user_hubs.push_back(net_man.join_server(SERVER_ID));
                                // // this should automatically display the Hub in the dropdown next
                              }
                              if (menubutton_selected == 2){
                                profile_menu_open = true;
                              }
                            }
                            return ret; });
  menubutton_menu |= CatchEvent([&](auto event)
                                {
    bool ret = Event::ArrowDown == event;
    if (ret){
      if (menubutton_selected < 2){
        menubutton_selected += 1;
      }
      
    }
    return ret; });
  menubutton_menu |= CatchEvent([&](auto event)
                                {
    bool ret = Event::ArrowUp == event;
    if (ret){
      if (menubutton_selected > 0){
        menubutton_selected -= 1;
      }
      
    }
    return ret; });

  auto components = Container::Horizontal({text_field, hub_dropdown, channel_dropdown, menubutton_menu});
  auto renderer = Renderer(components, [&]
                           {
                              std::string dummy_username = "kek";
>>>>>>> Stashed changes
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
<<<<<<< Updated upstream
                             return gridbox({
                                 {text_area | ftxui::flex_grow | yframe, hub_dropdown->Render(), channel_dropdown->Render()},
                                 {text_field->Render() | border}
                             }); });
  screen.Loop(renderer);
}
=======
                            if (!menubutton_open){
                             return gridbox({
                                 {text_area | ftxui::flex_grow | yframe, hub_dropdown->Render(), channel_dropdown->Render()},
                                 {text_field->Render() | border}
                             });}
                          else if (profile_menu_open) {
                            return gridbox({
                                {profile_window->Render()},
                                {text("If you're hamzaa you suck")},
                                {text("Username is " + dummy_username)},
                                {text("Email: avob@cutie.com")},
                                {text("Press <TAB> to exit")},
                                });
                          }
                            else {
                             return gridbox({
                                {menubutton_menu->Render()}
                                });
                            } });
  screen.Loop(renderer);
}
>>>>>>> Stashed changes
