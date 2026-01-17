#pragma once
#include "config_manager.hpp"
#include "main_window.hpp"
#include <memory>
#include <wx/ipc.h>
#include <wx/snglinst.h>

class paperback_connection : public wxConnection {
public:
	bool OnExec(const wxString& topic, const wxString& data) override;
};

class paperback_server : public wxServer {
public:
	wxConnectionBase* OnAcceptConnection(const wxString& topic) override;
};

class paperback_client : public wxClient {
};

class app : public wxApp {
public:
	app() = default;
	~app() = default;
	app(const app&) = delete;
	app& operator=(const app&) = delete;
	app(app&&) = delete;
	app& operator=(app&&) = delete;
	bool OnInit() override;
	int OnExit() override;
	void parse_command_line();
	void restore_previous_documents();
	[[nodiscard]] config_manager& get_config_manager() {
		return config_mgr;
	}
	void open_file(const wxString& filename);
	void check_for_updates(bool silent);

private:
	main_window* frame{nullptr};
	config_manager config_mgr;
	std::unique_ptr<wxSingleInstanceChecker> single_instance_checker;
	std::unique_ptr<paperback_server> ipc_server;
	void load_default_config();
};

wxDECLARE_APP(app);
