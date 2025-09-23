
#define RECT_PACK_H_IMPL
extern "C" {
#include "rect_pack.h"
}

#include "quick.h"

#include <string>
#include <cstdio>
#include <vector>
#include <random>

class RectApp : public quick::Application
{
    std::vector<rect_r> rects;
    ImVec2 old_sz = ImVec2(0.f, 0.f);
    bool packed = false;
    int page = 0;
    bool should_page = true;
    bool changed = false;
    int wrange[2] = {10, 100}, hrange[2] = {10, 100};
    int rnum = 100;

    void start(const Config& config)
    {
    }
    
    void update()
    {
        int packed_area = 0;
        int nobj = 0;

        ImGui::SetNextWindowPos(ImVec2(0.0f, 0.0f));

        auto wsize = ImGui::GetIO().DisplaySize;
        ImGui::SetNextWindowSize(wsize);

        ImGui::Begin("Rect", nullptr, 
            ImGuiWindowFlags_NoBackground | 
            ImGuiWindowFlags_NoBringToFrontOnFocus | 
            ImGuiWindowFlags_NoCollapse | 
            ImGuiWindowFlags_NoDecoration);
        auto* draw = ImGui::GetWindowDrawList();

        ImVec2 p0 = ImGui::GetCursorScreenPos();
        ImVec2 sz = ImGui::GetContentRegionAvail();
        ImVec2 p1 = ImVec2(p0.x + sz.x, p0.y + sz.y);

        if(changed) {
            packed = rect_pack(sz.x, sz.y, should_page, rects.data(), rects.size());
            changed = false;
        }

        draw->AddRect(p0, p1, IM_COL32(0x00, 0xff, 0xff, 255), 0, 0, 2);

        for(int i = 0; i < rects.size(); i++) {
            rect_r& r = rects[i];

            if(r.info.page == page && r.info.packed) {
                
                auto topleft = ImVec2(p0.x + r.info.x, p0.y + r.info.y);

                draw->AddRect(topleft, 
                    ImVec2(topleft.x + r.w, topleft.y + r.h), 
                    IM_COL32(0xff, 0x00, 0xff, 255));      

                packed_area += r.h * r.w;      
                nobj++;
            }
        }

        auto vp = ImGui::GetMainViewport();
        auto size = vp->Size;

        ImGui::End();

        if(old_sz.x != sz.x || old_sz.y != sz.y) {
            changed = true;
        }
        old_sz = sz;
    
        ImGui::Begin("Controls");

        ImGui::Text("page size: %.0fx%.0f", sz.x, sz.y);
        ImGui::Text("page: %d", page); 
        
        if(ImGui::Checkbox("paging", &should_page)) {
            changed = true;
        }

        ImGui::SameLine();
        if(ImGui::Button("prev")) {
            page--;
        }
        ImGui::SameLine();
        if(ImGui::Button("next")) {
            page++;
        }

        ImGui::Text("objects: %d/%d", nobj, static_cast<int>(rects.size()));

        ImGui::Text("packing: %.3f", 
            static_cast<float>(packed_area) / static_cast<float>(sz.x * sz.y));

        ImGui::SeparatorText("rects");

        bool add_rects = ImGui::Button("add random");
        ImGui::SameLine();
        if(ImGui::Button("clear")) {
            rects.clear();
            changed = true;
        }
    
        ImGui::InputInt2("width range", wrange);
        ImGui::InputInt2("height range", hrange);
        ImGui::InputInt("number", &rnum);

        if(rnum < 0) rnum = 0;
        if(wrange[0] < 1) wrange[0] = 1;
        if(wrange[1] < 1) wrange[1] = 1;
        if(hrange[0] < 1) hrange[0] = 1;
        if(hrange[1] < 1) hrange[1] = 1;

        if(add_rects) {

            std::random_device rd;
            std::mt19937 gen(rd());
            std::uniform_int_distribution<> wd(wrange[0], wrange[1]);
            std::uniform_int_distribution<> hd(hrange[0], hrange[1]);
    
            for(int i = 0; i < rnum; i++) {
                rect_r r;
                r.w = wd(gen);
                r.h = hd(gen);
                r.id = rects.size();
                rects.push_back(r);
            }

            changed = true;
        }

        ImGui::End();
    }

};

int main() {

    auto app = RectApp();
    
    auto config = quick::Application::Config();
    config.name = "rect_pack.h test";
    config.w = 800;
    config.h = 600;
    config.render_on_resize = true;

    if(!app.setup(config))
    {
        return 1;
    }
    app.run();

    return 0;
}