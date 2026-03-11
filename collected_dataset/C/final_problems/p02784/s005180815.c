#include <stdio.h>

int main(){
    int h1, n, h2 = 0, cont = 0;

    scanf("%d %d", &h1, &n);

    int v[n];

    for(int i = 0; i < n; ++i){
        scanf("%d", &v[i]);
    }
    for(int i = 0; i < n; ++i){
        for(int j = i + 1; j < n; ++j){
            if(v[i] == v[j]){
                cont += v[i];
            }
        }
    }

    for(int i = 0; i < n; ++i){
        h2 += v[i];
    }
    h2 -= cont;

    if(h2 >= h1){
        printf("Yes\n");
    }else{
        printf("No\n");
    }


    return 0;
}
