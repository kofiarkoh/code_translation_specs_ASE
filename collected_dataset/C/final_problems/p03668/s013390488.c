#include <stdio.h>
#include <stdlib.h>
int x[100000], y[100000];
int cnt[100000], cnts[100000], *tree[100000];
int dfs(int a, int b){
    if(cnt[a] == 1){//何も繋がっていない頂点,
        return 0;
    }
    int i;
    int g_list[100000];
    for(i = 0; i < cnt[a]; i++){
        g_list[i] = 0;
    }
    for(i = 0; i < cnt[a]; i++){
        if(tree[a][i] == b){
            continue;
        }
        g_list[dfs(tree[a][i], a)] = 1;
    }
    for(i = 0; i < 100000; i++){
        if(g_list[i] == 0){
            return i;
        }
    }
}
int main(void){
    int n, i, j;
    scanf("%d",&n);
    for(i = 0; i < n; i++){
        cnt[i] = 0;
        cnts[i] = 0;
    }
    for(i = 0; i < n-1; i++){
        scanf("%d%d", &x[i], &y[i]);
        x[i]--;
        y[i]--;
        cnt[x[i]]++;
        cnt[y[i]]++;
    }
    for(i = 0; i < n; i++){
        tree[i] = malloc((cnt[i] + 1) * sizeof(int));
    }
    for(i = 0; i < n-1; i++){
        tree[x[i]][cnts[x[i]]++] = y[i];
        tree[y[i]][cnts[y[i]]++] = x[i];
    }
    int ans = 0;
    for(i = 0; i < cnt[0]; i++){
        ans ^= dfs(tree[0][i], 0);
    }


    if(ans == 0){
        printf("Bob\n");
    }else{
        printf("Alice\n");
    }
    return 0;
}