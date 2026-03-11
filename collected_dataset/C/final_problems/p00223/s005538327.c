#include <stdio.h>
#include <string.h>

typedef struct {
    int ax, ay;
    int bx, by;
    int time;
} TWIN;

TWIN queue[300000];
int head, tail;

void enq(TWIN t)
{
    queue[tail % 300000] = t;
    tail++;
}

void deq(TWIN *t)
{
    *t = queue[head % 300000];
    head++;
}

int main(void)
{
    TWIN first, temp;
    int x, y;
    int i, j;
    int dx[4] = {1, 0, -1, 0}, dy[4] = {0, 1, 0, -1};
    int map[50][50];
    int v[50][50];
    int flag;
    
    while (1){
        scanf("%d%d", &x, &y);
        
        if (x + y == 0){
            break;
        }
        
        scanf("%d%d", &first.ax, &first.ay);
        scanf("%d%d", &first.bx, &first.by);
        
        first.ax -= 1;
        first.ay -= 1;
        first.bx -= 1;
        first.by -= 1;
        
        for (i = 0; i < y; i++){
            for (j = 0; j < x; j++){
                scanf("%d", &map[i][j]);
            }
        }
        first.time = temp.time = 0;
        head = tail = 0;
        memset(v, 0, sizeof(v));
        enq(first);
        flag = 0;
        while (head != tail && temp.time <= 100){
            deq(&temp);
            
            if (temp.ax == temp.bx && temp.ay == temp.by){
                flag = 1;
                break;
            }
            
            v[temp.ay][temp.ax] |= 2;
            v[temp.by][temp.bx] |= 1;
            
            for (i = 0; i < 4; i++){
                first = temp;
                if (0 <= first.ax + dx[i] && first.ax + dx[i] < x && 0 <= first.ay + dy[i] && first.ay + dy[i] < y && map[first.ay + dy[i]][first.ax + dx[i]] == 0){
                    first.ax += dx[i];
                    first.ay += dy[i];
                }
                if (0 <= first.bx - dx[i] && first.bx - dx[i] < x && 0 <= first.by - dy[i] && first.by - dy[i] < y && map[first.by - dy[i]][first.bx - dx[i]] == 0){
                    first.bx -= dx[i];
                    first.by -= dy[i];
                }
                first.time = temp.time + 1;
                if (v[first.ay][first.ax] < 2 && v[first.bx][first.by] != 1){
                    enq(first);
                }
            }
        }
        if (flag == 1){
            printf("%d\n", temp.time);
        }
        else {
            printf("NA\n");
        }
    }
    
    return (0);
}