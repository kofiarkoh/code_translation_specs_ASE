#include<stdio.h>

int main(){
    int n, m;
    scanf("%d%d", &n, &m);
    int count[n-1];
    for(int i=0; i<n-1; i++){
        count[i] = 0;
    }
    int a[m], b[m], f[m];
    for(int i=0; i<m; i++){
        scanf("%d%d", &a[i], &b[i]);
        a[i]--;
        b[i]--;
        f[i] = 1;
        for(int j=a[i]; j<b[i]; j++){
            count[j]++;
        }
    }
    int c = 0, flag = 1;
    while(flag){
        c++;
        int max = count[0], id = 0;
        for(int i=1; i<n-1; i++){
            if(max < count[i]){
                max = count[i];
                id = i;
            }
        }
        for(int i=0; i<m; i++){
            if(f[i]){
                if(a[i] <= id && id < b[i]){
                    for(int j=a[i]; j<b[i]; j++){
                        count[j]--;
                    }
                    f[i] = 0;
                }
            }
        }
        flag = 0;
        for(int i=0; i<m; i++){
            if(f[i]){
                flag = 1;
            }
        }
    }
    printf("%d", c);

    return 0;
}