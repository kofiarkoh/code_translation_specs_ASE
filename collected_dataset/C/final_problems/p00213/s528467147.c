#include <stdio.h>
#include <string.h>

int x, y, n;
int name[15], size[15];
int ans, record[10][10];

int dx[4] = {1, 0, -1, 0};
int dy[4] = {0, 1, 0, -1};

void copy(int saki[][10], int moto[][10])
{
    int i, j;
    
    for (i = 0; i < y; i++){
        for (j = 0; j < x; j++){
            saki[i][j] = moto[i][j];
        }
    }
}

int canPlace(int w, int h, int sy, int sx, int now, int map[][10])
{
    int i, j;
    int flag = 0;
    
    for (i = sy; i < sy + h; i++){
        for (j = sx; j < sx + w; j++){
            if (map[i][j] == name[now]){
                flag = 1;
            }
            else if (map[i][j]){
                return (0);
            }
        }
    }
    return (flag);
}

void place(int w, int h, int sy, int sx, int now, int map[][10])
{
    int i, j;
    
    for (i = sy; i < sy + h; i++){
        for (j = sx; j < sx + w; j++){
            map[i][j] = name[now];
        }
    }
}

int generateMap(int now, int map[][10])
{
    int i, j, k, t;
    int my, mx;
    int temp[10][10];
    int check;
    int w, h;
    
    
    if (now == n){
        copy(record, map);
        ans++;
        return (1);
    }

    check = 0;
    for (t = 1; t <= size[now]; t++){
        if (size[now] % t != 0){
            continue;
        }
        w = size[now] / t;
        h = t;
        
        for (i = 0; i <= y - h; i++){
            for (j = 0; j <= x - w; j++){
                if (canPlace(w, h, i, j, now, map)){
                    copy(temp, map);
                    place(w, h, i, j, now, temp);
                    check |= generateMap(now + 1, temp);
                }
            }
        }
    }
    
    return (check);
}

int main(void)
{
    
    int i, j;
    int map[10][10];
    
    while (1){
        scanf("%d%d%d", &x, &y, &n);
        
        if (x + y + n == 0){
            break;
        }
        
        for (i = 0; i < n; i++){
            scanf("%d%d", &name[i], &size[i]);
        }
        for (i = 0; i < y; i++){
            for (j = 0; j < x; j++){
                scanf("%d", &map[i][j]);
            }
        }
        
        memset(record, 0, sizeof(record));
        ans = 0;
        
        if (!generateMap(0, map) || ans != 1){
            printf("NA\n");
        }
        else {
            for (i = 0; i < y; i++){
                for (j = 0; j < x; j++){
                    printf("%d%c", record[i][j], j == x - 1 ? '\n' : ' ');
                }
            }
        }
    }
    
    return (0);
}