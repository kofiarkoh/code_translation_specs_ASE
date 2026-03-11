#include <stdio.h>
int main(void){
    // Your code here!
    int town[4]={0,0,0,0},num1,num2,cnt=0;
    
    scanf("%d %d",&num1,&num2);
    town[num1-1]++; town[num2-1]++;
    scanf("%d %d",&num1,&num2);
    town[num1-1]++; town[num2-1]++;
    scanf("%d %d",&num1,&num2);
    town[num1-1]++; town[num2-1]++;
    
    for(int i=0;i<4;i++){
        if(town[i]%2==1){
            cnt++;
        }
    }
    
    if(cnt==0 || cnt==2){
        printf("YES\n");
    }else{
        printf("NO\n");
    }
}
