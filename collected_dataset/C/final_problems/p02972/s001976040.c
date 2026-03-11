#include<stdio.h>
int main()
{
    int n;
    int flag;
    int s[100];
    scanf("%d",&n);
    for(int i=0;i<n;i++){
        scanf("%d",&s[i]);
    }
    if(s[0]==1){
        flag=1;
        for(int i=1;i<n;i++){
            for(int j=i+1;j<n;j++){
                if(s[i]==1 && s[j]==1 && (j+1)%(i+1)!=0){
                    flag=0;
                    printf("-1\n");
                    break;
                }
            }
            if(flag==0){
                break;
            }
        }
        if(flag==1){
            printf("1\n");
            printf("1\n");
        }
    }
    else{
        flag=1;
        for(int i=0;i<n;i++){
            if(s[i]!=0){
                printf("-1\n");
                flag=0;
                break;
            }
        }
        if(flag==1){
            printf("0\n");
        }
    }
    return 0;
}
