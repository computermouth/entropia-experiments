
typedef struct {
    float x;
    float y;
    float z;
} rb_vec3;

typedef struct {
    float pos[3];
    float size[3];
    unsigned char fcolor[4];
    unsigned char wcolor[4];
} rb_node;

void game_init(int width, int height, const char * name);
void round_init(float x, float y, float z, float fov);
void update_camera(rb_vec3 mov, rb_vec3 rot, float zoom);
void begin_drawing();
void draw_node(rb_node n);
void end_drawing();
int window_should_close();