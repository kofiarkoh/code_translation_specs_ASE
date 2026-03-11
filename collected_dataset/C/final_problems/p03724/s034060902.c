#include <stdio.h>

int main(void){
    int slot[100001] = {0};
    int flag = 0;
    int n, m, a, b;
    scanf("%d %d", &n, &m);
    for(int i = 0;i < m;i++){
        scanf("%d %d", &a, &b);
        slot[a] = !slot[a];
        slot[b] = !slot[b];
    }
    for(int i = 0;i < n;i++){
        if(slot[i] == 1) flag = 1;
    }
    if(flag == 1) printf("NO\n");
    else printf("YES\n");
}