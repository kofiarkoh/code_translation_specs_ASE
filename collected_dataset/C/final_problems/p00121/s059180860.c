#include <stdio.h>
#include <string.h>

typedef struct {
    int num[2][4];
    int time;
    int hash;
    int x, y;
} PUZZLE;

PUZZLE queue[50000];
int head, tail;

void enq(PUZZLE t)
{
    queue[(tail++) % 50000] = t;
}

void deq(PUZZLE *t)
{
    *t = queue[(head++) % 50000];
}

int generateHash(int num[][4])
{
    int ret;
    int i;
    ret = 0;
    for (i = 0; i < 8; i++){
        ret = ret * 10 + num[i / 4][i % 4];
    }
    return (ret);
}

int main(void)
{
    PUZZLE first, temp;
    char str[30];
    int i, j;
    int dx[4] = {1, 0, -1, 0};
    int dy[4] = {0, 1, 0, -1};
    static char v[76543210];
    
    while (scanf("%[^\n]", str) != EOF){
        getchar();
        for (i = 0; i < 2; i++){
            for (j = 0; j < 4; j++){
                first.num[i][j] = str[j * 2 + i * 8] - '0';
                if (first.num[i][j] == 0){
                    first.x = j;
                    first.y = i;
                }
            }
        }
        
        memset(v, 0, sizeof(v));
        
        first.time = 0;
        head = tail = 0;
        first.hash = generateHash(first.num);
        enq(first);
        
        while (1){
            deq(&temp);
            
            if (temp.hash == 1234567){
                break;
            }
            
            for (i = 0; i < 4; i++){
                first = temp;
                first.time = temp.time + 1;
                first.x += dx[i];
                first.y += dy[i];
                if (0 <= first.x && first.x < 4 && 0 <= first.y && first.y < 2){
                    first.num[temp.y][temp.x] ^= first.num[first.y][first.x];
                    first.num[first.y][first.x] ^= first.num[temp.y][temp.x];
                    first.num[temp.y][temp.x] ^= first.num[first.y][first.x];
                    first.hash = generateHash(first.num);
                    if (!v[first.hash]){
                        v[first.hash] = 1;
                        enq(first);
                    }
                }
            }
        }
        printf("%d\n", temp.time);
    }
    
    return (0);
}