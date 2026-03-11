#include <stdio.h>
#include <string.h>

typedef struct {
    int b, tx;
    int time;
} TOWER;

TOWER queue[400];

int head, tail;

void enq(TOWER t)
{
    queue[(tail++) % 400] = t;
}

void deq(TOWER *t)
{
    *t = queue[(head++) % 400];
}

int main(void)
{
    TOWER first, temp;
    int building[2][100];
    char v[2][100];
    int n;
    int i;
    char flag;
    
    while (1){
        scanf("%d", &n);
        
        if (n == 0){
            break;
        }
        
        for (i = 0; i < n; i++){
            scanf("%d", &building[0][i]);
        }
        for (i = 0; i < n; i++){
            scanf("%d", &building[1][i]);
        }
        
        memset(v, 0, sizeof(v));
        
        head = tail = 0;
        first.b = 0;
        first.tx = 0;
        first.time = 0;
        while (building[0][first.tx] == 1){
            v[0][first.tx++] = 1;
        }
        if (first.tx != 0){
            first.tx--;
        }
        enq(first);
        
        first.b = 1;
        first.tx = 0;
        while (building[1][first.tx] == 1){
            v[1][first.tx++] = 1;
        }
        if (first.tx != 0){
            first.tx--;
        }
        enq(first);
        
        flag = 0;
        
        while (head != tail){
            deq(&temp);
            
            if (temp.tx == n - 1){
                printf("%d\n", temp.time);
                flag = 1;
                break;
            }
            
            for (i = 0; i <= 2; i++){
                first = temp;
                first.b ^= 1;
                if (first.tx + i < n && !v[first.b][first.tx + i] && building[first.b][first.tx + i] == 0){
                    v[first.b][first.tx + i] = 1;
                    first.time++;
                    first.tx = temp.tx + i;
                    enq(first);
                }
                else if (first.tx + i < n && !v[first.b][first.tx + i] && building[first.b][first.tx + i] == 1){
                    first.time++;
                    first.tx = temp.tx + i;
                    while (building[first.b][first.tx] == 1 && first.tx < n && !v[first.b][first.tx]){
                        v[first.b][first.tx++] = 1;
                    }
                    if (first.tx != temp.tx){
                        first.tx--;
                    }
                    enq(first);
                }
                else if (first.tx + i < n && !v[first.b][first.tx + i] && building[first.b][first.tx + i] == 2){
                    first.tx = temp.tx + i;
                    first.time++;
                    while (building[first.b][first.tx] == 2 && first.tx < n && !v[first.b][first.tx]){
                        v[first.b][first.tx--] = 1;
                    }
                    if (building[first.b][first.tx] != 2){
                        enq(first);
                    }
                }
            }
        }
        if (!flag){
            printf("NA\n");
        }
    }
    
    return (0);
}