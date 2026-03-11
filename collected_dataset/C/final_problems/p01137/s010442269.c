#include <stdio.h>
#include <math.h>

int min(int a, int b)
{
    if (a > b){
        return (b);
    }
    return (a);
}

int ans;
int dfs(int m, int depth, int s)
{
    int i;
    if (depth == 1){
        return (m + s);
    }
    
    for (i = 0; pow(i, depth) <= m; i++){
        ans = min(ans, dfs(m - pow(i, depth), depth - 1, s + i));
    }
    
    return (ans);
}

int main(void)
{
    int m, n;
    int i, j, k;
    
    while (1){
        scanf("%d", &m);
        
        if (m == 0){
            break;
        }
        
        ans = m;
        printf("%d\n", dfs(m, 3, 0));
    }
    
    return (0);
}