#include <stdio.h>
#include <string.h>

typedef struct {
    int x, y;
} POINT;

int x, y;
char map[2][1024][1024];
char visit[1024][1024];
int head, tail;
POINT p[1024], queue[100000];

void enq(POINT t)
{
    queue[(tail++) % 100000] = t;
}

POINT deq(void)
{
    return (queue[(head++) % 100000]);
}

int bfs(void)
{
    POINT add, temp;
    int dx[] = {1, 0, -1, 0};
    int dy[] = {0, 1, 0, -1};
    int i;
    
    memset(visit, 0, sizeof(visit));
    head = tail = 0;
    add.x = add.y = 0;
    visit[0][0] = 1;
    enq(add);
    
    while (head != tail){
        temp = deq();
        
        if (map[1][add.y][add.x] == 't'){
            return (1);
        }
        
        for (i = 0; i < 4; i++){
            add.x = temp.x + dx[i];
            add.y = temp.y + dy[i];
            
            if (0 <= add.x && add.x < x && 0 <= add.y && add.y < y && !visit[add.y][add.x] && map[1][add.y][add.x] != '#'){
                visit[add.y][add.x] = 1;
                enq(add);
            }
        }
    }
    
    return (0);
}

int main(void)
{
    int i, j;
    int left, right, center;
    int n;
    
    scanf("%d%d", &x, &y);
    
    for (i = 0; i < y; i++){
        scanf("%s", map[0][i]);
        strcpy(map[1][i], map[0][i]);
    }
    
    p[0].x = p[0].y = 0;
    
    scanf("%d", &n);
    
    for (i = 1; i <= n; i++){
        scanf("%d%d", &p[i].x, &p[i].y);
    }
    
    left = 0;
    right = n;
    center = 0;
    
    while (left != right){
        if (center > (left + right) / 2){
            for (i = center; i > (left + right) / 2; i--){
                if (map[0][p[i].y][p[i].x] == '#'){
                    map[1][p[i].y][p[i].x] = '#';
                }
            }
        }
        else {
            for (i = center; i <= (left + right) / 2; i++){
                map[1][p[i].y][p[i].x] = '.';
            }
        }
        center = (left + right) / 2;
        if (bfs()){
            right = center;
        }
        else {
            left = center + 1;
        }
    }
    
    if (center > (left + right) / 2){
        for (i = center; i > (left + right) / 2; i--){
            if (map[0][p[i].y][p[i].x] == '#'){
                map[1][p[i].y][p[i].x] = '#';
            }
        }
    }
    else {
        for (i = center; i <= (left + right) / 2; i++){
            map[1][p[i].y][p[i].x] = '.';
        }
    }
    
    if (bfs()){
        printf("%d\n", left);
    }
    else {
        printf("-1\n");
    }
    
    return (0);
}