#include <stdio.h>

int main(){
        int n;
        int p[30000];

        int s = 0;

        scanf("%d", &n);
        for(int i = 1; i <= n; i++) scanf("%d", &p[i]);
        
        for (int i = 1; i <= n; i++){
                if(p[i]%2 == 0 && i%2 == 0) s = 0;
                else if (p[i]%2 == 1 && i%2 == 1) s = 0;
                else {
                        s = 1;
                        break;
                }
        }
        
        if(s == 0) printf("Yes\n");
        else printf("No\n");
        
        return 0;
}       

