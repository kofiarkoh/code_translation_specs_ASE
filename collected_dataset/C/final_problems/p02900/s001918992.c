#include <stdio.h>
int main(){
    int a,b,count=0,result=0;
    int check =1;
    scanf("%d %d",&a,&b);
    int p[(a<b)?a:b];
    //素数判定
    for (int i=1;i<=((a<b)?a:b);i++){
        //printf("count:%d\n",count);
//iが素数かしらべる
    for (int j=1;j<count;j++){
        //iが素数か調べるために、既知の素数列pを用いる
        //printf("i:%d p[%d]:%d\n",i,j,p[j]);
        if(i%p[j]==0){
           // printf("%dは素数でない\n",i);
          check=0;
            break;//iは素数でない//下の処理を飛ばしiを増やす
        }
        check=1;
        }
    
    if (check==1){
        //以下は素数を追加する処理
        //printf("%dを%d番目の素数として認める\n",i,count+1);
        p[count]=i;
        // for(int k=0;k<=count;k++){
        //     printf("%d番目の素数は %d\n",k+1,p[k]);    
        // }        
        count++;
        check =1;
        //ここまで素数追加
        
        if (a%i==0&&b%i==0){
            //printf("c\n");
            //printf("素数%dは%d,%dをともにわれます\n",i,a,b);
            result++;
        }
    }
}
    printf("%d",result);
    return 0;
}