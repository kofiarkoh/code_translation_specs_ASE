#include <stdio.h>
#include <string.h>

typedef struct {
    int ax, ay;
    int bx, by;
    int time;
    int g_pat;
} GHOST;

GHOST queue[500000];
int head, tail;

void enq(GHOST t)
{
    queue[tail % 100000] = t;
    tail++;
}

void deq(GHOST *t)
{
    *t = queue[head % 100000];
    head++;
}

int main(void)
{
    int x, y;
    int i, j;
    int len;
    char map[21][21];
    char pattern[11];
    static char v[21][21][21][21][10];
    GHOST first, temp;
    char flag;
    int dx[4] ={1, 0, -1, 0}, dy[4] = {0, 1, 0, -1};
    int move[5][2] = {{0, 0}, {0, -1}, {1, 0}, {-1, 0}, {0, 1}};
    while (1){
        scanf("%d%*c%d", &y, &x);
        getchar();
        
        if (x + y == 0){
            break;
        }
        
        memset(map, '\0', sizeof(map));
        
        for (i = 0; i < y; i++){
            for (j = 0; j < x; j++){
                scanf("%c", &map[i][j]);
                if (map[i][j] == 'A'){
                    first.ax = j;
                    first.ay = i;
                    map[i][j] = '.';
                }
                else if (map[i][j] == 'B'){
                    first.bx = j;
                    first.by = i;
                    map[i][j] = '.';
                }
            }
            getchar();
        }
        scanf("%s", pattern);
        getchar();
        len = strlen(pattern);
        for (i = 0; i < len; i++){
            switch (pattern[i]){
                case '5':
                pattern[i] = 0;
                break;
                
                case '8':
                pattern[i] = 1;
                break;
                
                case '6':
                pattern[i] = 2;
                break;
                
                case '4':
                pattern[i] = 3;
                break;
                
                case '2':
                pattern[i] = 4;
                break;
            }
        }
        first.time = first.g_pat = head = tail = 0;
        memset(v, 0, sizeof(v));
        enq(first);
        flag = 0;
        while (head != tail){
            deq(&temp);
            
            if (temp.ax == temp.bx && temp.ay == temp.by){
                flag = 1;
                break;
            }
            
            for (i = 0; i < 4; i++){
                first = temp;
                if (0 <= temp.ax + dx[i] && temp.ax + dx[i] < x && 0 <= temp.ay + dy[i] && temp.ay + dy[i] < y && map[temp.ay + dy[i]][temp.ax + dx[i]] == '.'){
                    first.ax = temp.ax + dx[i];
                    first.ay = temp.ay + dy[i];
                }
                
                if (0 <= temp.bx + move[pattern[temp.g_pat]][0] && temp.bx + move[pattern[temp.g_pat]][0] < x && 0 <= temp.by + move[pattern[temp.g_pat]][1] && temp.by + move[pattern[temp.g_pat]][1] < y){
                    first.bx = temp.bx + move[pattern[temp.g_pat]][0];
                    first.by = temp.by + move[pattern[temp.g_pat]][1];
                }
                first.g_pat = temp.g_pat + 1;
                first.g_pat %= len;
                first.time = temp.time + 1;
                if (!v[first.ax][first.ay][first.bx][first.by][first.g_pat]){
                    v[first.ax][first.ay][first.bx][first.by][first.g_pat] = 1;
                    enq(first);
                }
            }
        }
        
        if (flag == 1){
            printf("%d %d %d\n", temp.time, temp.ay, temp.ax);
        }
        
        else {
            printf("impossible\n");
        }
    }
    
    return (0);
}