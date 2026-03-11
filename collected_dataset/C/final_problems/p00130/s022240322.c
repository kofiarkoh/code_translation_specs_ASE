#include <stdio.h>
#include <ctype.h>
int main(void){
    int n,i,o,min,max,t;
    char line[1025],line1[60];
    scanf("%d",&n);
    for(i=0;min=t=30,max=31,i<n;i++){
        scanf("%s",line);
        line1[min]=line[0];
        for(o=3;line[o];o++)
            if(isalpha(line[o])){
                if(line[o-1] == '>'){
                    line1[++t]=line[o];
                    if(max==t)
                        max++;
                }
                else{
                    if(min==t)
                        min--;
                    line1[--t]=line[o];
                }
            }
        line1[max]=0;
        printf("%s\n",line1+min);
    }
    return 0;
}