#include <stdio.h>
#include <string.h>

typedef struct{
    int price,weight;
    char type;
    char date[13+1];
    char name[20+1];
}Shina;

void swap(Shina s[],int i,int j){
    Shina tmp;
    tmp=s[i];
    s[i]=s[j];
    s[j]=tmp;
    return;
}
void sort(Shina s[],int i){
    int j;
    for(j=i;j>0;j--){
        if(s[j].price<s[j-1].price)swap(s,j,j-1);
        else if(s[j].price==s[j-1].price){
            if(s[j].weight<s[j-1].weight)swap(s,j,j-1);
            else if(s[j].weight==s[j-1].weight){
                if(s[j].type<s[j-1].type)swap(s,j,j-1);
                else if(s[j].type==s[j-1].type){
                    if(strcmp(s[j].date,s[j-1].date)<0)swap(s,j,j-1);
                    else if(strcmp(s[j].date,s[j-1].date)==0){
                        if(strcmp(s[j].name,s[j-1].name)<0)swap(s,j,j-1);
                    }
                }
            }
        }
    }
    return;
}
int main(){
    int i;
    
    int n;
    scanf("%d\n",&n);
    
    Shina s[n];
    for(i=0;i<n;i++){
        scanf("%d %d %c %s %s\n",&s[i].price,&s[i].weight,&s[i].type,s[i].date,s[i].name);
        sort(s,i);
    }
    
    for(i=0;i<n;i++)printf("%d %d %c %s %s\n",s[i].price,s[i].weight,s[i].type,s[i].date,s[i].name);
    
    return 0;
}
