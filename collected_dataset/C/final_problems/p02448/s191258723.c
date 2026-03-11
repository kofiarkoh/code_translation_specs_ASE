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
    int j=i;
    while(j>0&&s[i].price<s[j-1].price)j--;
    if(s[i].price==s[j-1].price){
        while(j>0&&s[i].price==s[j-1].price&&s[i].weight<s[j-1].weight)j--;
        if(s[i].price==s[j-1].price&&s[i].weight==s[j-1].weight){
            while(j>0&&s[i].weight==s[j-1].weight&&s[i].type<s[j-1].type)j--;
            if(s[i].price==s[j-1].price&&s[i].weight==s[j-1].weight&&s[i].type==s[j-1].type){
                while(j>0&&s[i].type==s[j-1].type&&strcmp(s[i].date,s[j-1].date)<0)j--;
                if(s[i].price==s[j-1].price&&s[i].weight==s[j-1].weight&&s[i].type==s[j-1].type&&strcmp(s[i].date,s[j-1].date)==0){
                    while(j>0&&strcmp(s[i].date,s[j-1].date)==0&&strcmp(s[i].name,s[j-1].name)<0)j--;
                }
            }
        }
    }
    
    if(j==i)return;
    
    Shina tmp=s[i];
    int k;
    for(k=i;k>j;k--)s[k]=s[k-1];
    s[j]=tmp;
    
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
