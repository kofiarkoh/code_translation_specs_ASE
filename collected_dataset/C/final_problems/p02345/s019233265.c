#include <stdio.h>
#include <stdlib.h>

#define N_MAX 100000
#define Q_MAX 100000
#define A_MAX 0x7FFFFFFF

typedef unsigned int    _uint;
typedef unsigned short  _ushort;

typedef struct{
    _ushort com;
    _uint   x;
    _uint   y;
}Que;

void que_operator(Que que, _uint n);

int main(void)
{
    _uint   n, q;
    _uint   ii;
    
    Que     que[Q_MAX];
    
    scanf("%d %d", &n, &q);
    
    for(ii = 0; ii < q; ii++){
        scanf("%d %d %d", &(que[ii].com), &(que[ii].x), &(que[ii].y));
    }
    
    for(ii = 0; ii < q; ii++){
        que_operator(que[ii], n);
    }
}

void que_operator(Que que, _uint n)
{
    static _uint    a[N_MAX];
    static _uint    call_list[Q_MAX];
    static _uint    call_count = 0;
    static _uint    call_count_cache = 0;
    static Que      que_cache;
    static _uint    result_cache;
    
    _uint           ii;
    _uint           r1, r2;
    _uint           result;
    
    switch(que.com){
        case 0:
            if(call_count == 0){
                for(ii = 0; ii < n; ii++){
                    a[ii] = A_MAX;
                }
            }
            a[que.x] = que.y;
            call_list[call_count] = que.x;
            call_count++;
            break;
        case 1:
            if(call_count != 0){
                if(call_count == call_count_cache){
                    if(que_cache.com == que.com){
                        if(que_cache.x == que.x){
                            if(que_cache.y == que.y){
                                printf("%d\n", result_cache);
                                return;
                            }
                        }
                    }
                }
                result = A_MAX;
                if(que.x > que.y){
                    r1 = que.y;
                    r2 = que.x;
                }else{
                    r1 = que.x;
                    r2 = que.y;
                }
                if(1){
                    for(ii = r1; ii <= r2; ii++){
                        if(result > a[ii]){
                            result = a[ii];
                        }
                    }
                }else{
                    for(ii = 0; ii < call_count; ii++){
                        if(result > a[call_list[ii]]){
                            if(r1 <= call_list[ii] && call_list[ii] <= r2){
                                result = a[call_list[ii]];
                            }
                        }
                    }
                }
                que_cache = que;
                result_cache = result;
            }else{
                result = A_MAX;
                que_cache = que;
                result_cache = result;
            }
            call_count_cache = call_count;
            printf("%d\n", result);
            break;
        default:
            break;
    } 
}
