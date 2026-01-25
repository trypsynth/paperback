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
