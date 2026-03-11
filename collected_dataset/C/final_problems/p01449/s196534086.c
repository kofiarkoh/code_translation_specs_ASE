#include <stdio.h>
#include <string.h>

typedef struct {
    int tx;
    int time;
} DICE;
DICE queue[100000];
int head, tail;

void enq(DICE t)
{
    queue[(tail++) % 100000] = t;
}

void deq(DICE *t)
{
    *t = queue[(head++) % 100000];
}

int main(void)
{
    DICE first, temp;
    static int map[100000];
    static char v[100000];
    int now;
    int i;
    int n;
    
    scanf("%d", &n);
    
    for (i = 0; i < n; i++){
        scanf("%d", &map[i]);
    }
    
    memset(v, 0, sizeof(v));
    first.tx = first.time = 0;
    v[0] = 1;
    head = tail = 0;
    enq(first);
    
    while (head != tail){
        deq(&temp);
        
        if (temp.tx == n - 1){
            printf("%d\n", temp.time);
            break;
        }
        
        for (i = 1; i <= 6; i++){
            if (!v[temp.tx + i] && map[temp.tx] == 0){
                v[temp.tx + i] = 1;
                first.tx = temp.tx + i;
                first.time = temp.time + 1;
                enq(first);
            }
            else if (!v[temp.tx + i] && map[temp.tx + i] != 0){
                temp.tx += i;
                while (map[temp.tx] != 0 && !v[temp.tx + map[temp.tx]]){
                    v[temp.tx + map[temp.tx]] = 1;
                    temp.tx += map[temp.tx];
                }
                enq(temp);
            }
        }
    }
    
    return (0);
}