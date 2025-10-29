/* app.hpp - wxApp implementation header.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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

private:
	main_window* frame{nullptr};
	config_manager config_mgr;
	std::unique_ptr<wxSingleInstanceChecker> single_instance_checker;
	std::unique_ptr<paperback_server> ipc_server;
	void load_default_config();
};

wxDECLARE_APP(app);
