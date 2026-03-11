#include <stdio.h>
#include <stdlib.h>
#include <string.h>
typedef struct set{
    char name[11];
    int line[30];
    int sum;
}set;
int c(const void *v1,const void *v2){
    set x=*(set *)v1;
    set y=*(set *)v2;
    if(x.sum == y.sum)
        return strcmp(x.name,y.name);
    else
        return x.sum-y.sum;
}
set s[20];
int main(void){
    int n,line[30],i,o,temp,num;
    for(;memset(s,0,sizeof(s)),memset(line,0,sizeof(line)),scanf("%d",&n),n;){
        for(i=0;i<n;i++){
            scanf("%s %d",s[i].name,&num);
            for(o=0;o<num;o++){
                scanf("%d",&temp);
                line[temp]++;
                s[i].line[temp]++;
            }
        }
        for(i=0;i<n;i++)
            for(o=0;o<30;o++)
                if(s[i].line[o] != 0)
                    s[i].sum+=(n-line[o])+1;
        qsort(s,n,sizeof(set),c);
        printf("%d %s\n",s[0].sum,s[0].name);
    }
    return 0;
}