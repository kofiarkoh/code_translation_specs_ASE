#include <stdio.h>
#include <string.h>
int main(void){

    int n,m,max = 0;

    scanf("%d",&n);
    char red[n][11];
    for(int i=0;i<n;++i){
        scanf("%s",red[i]);
    }
    scanf("%d",&m);
    char blue[m][11];
    for(int i=0;i<m;++i){
        scanf("%s",blue[i]);
    }

    for(int i=0;i<n;++i){
        int count =0;
        for(int l=i;l<n;++l){
            if(strncmp(red[i],red[l],11) == 0){
                ++count;
            }
        }
        for(int p=0;p<m;++p){
            if(strncmp(red[i],blue[p],11) == 0){
                --count;
            }
        }
        if(max < count) max = count;
    }
    printf("%d",max);

    return 0;
}