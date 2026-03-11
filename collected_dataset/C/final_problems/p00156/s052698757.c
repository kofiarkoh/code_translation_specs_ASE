#include <stdio.h>
#include <string.h>

typedef struct {
    int ty;
    int tx;
    int sharp;
} POINT;

POINT queue[100000];
int head, tail;

void enq(POINT t)
{
    queue[tail++] = t;
    if (tail == 100000){
        tail = 0;
    }
}

void deq(POINT *t)
{
    *t = queue[head++];
    if (head == 100000){
        head = 0;
    }
}

int main(void)
{
    POINT temp, add;
    int ans;
    int i, j;
    int x, y;
    int mx, my;
    int dx[4] = {1, 0, -1, 0};
    int dy[4] = {0, 1, 0, -1};
    int cost[102][102];
    char map[102][103];
    
    while (1){
        scanf("%d%d", &x, &y);
        
        if (x + y == 0){
            break;
        }
        getchar();
        
        memset(map, '.', sizeof(map));
        
        for (i = 1; i <= y; i++){
            for (j = 1; j <= x; j++){
                scanf("%c", &map[i][j]);
                
                if (map[i][j] == '&'){
                    temp.tx = j;
                    temp.ty = i;
                }
            }
            getchar();
        }
        
        for (i = 0; i <= y; i++){
            for (j = 0; j <= x; j++){
                cost[i][j] = 100000;
            }
        }
        
        cost[temp.ty][temp.tx] = 0;
        temp.sharp = 0;
        head = tail = 0;
        enq(temp);
        ans = 100000;
        
        while (head != tail){
            deq(&temp);
            
            if (0 == temp.tx || 0 == temp.ty || x + 1 == temp.tx || y + 1 == temp.ty){
                ans = (ans > temp.sharp ? temp.sharp : ans);
                continue;
            }
            
            for (i = 0; i < 4; i++){
                mx = temp.tx + dx[i];
                my = temp.ty + dy[i];
                add = temp;
                if (cost[my][mx] > add.sharp){
                    if ((map[my][mx] == '#') && map[temp.ty][temp.tx] == '.'){
                        add.sharp++;
                    }
                    cost[my][mx] = add.sharp;
                    add.tx = mx;
                    add.ty = my;
                    enq(add);
                }
            }
        }
        
        printf("%d\n", ans);
    }
    
    return (0);
}