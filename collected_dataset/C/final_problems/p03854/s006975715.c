#include <stdio.h>
#include <string.h>

int i;
int main(void)
{
    char s[100001];
    char t[4][8]={"maerd","remaerd","esare","resare"};// <-4種
    char tmp;
    scanf("%s",s);
    int len_s=(int)strlen(s);
    for(int k=0;k<len_s/2;k++){
        tmp=s[k];
        s[k]=s[len_s-1-k];
        s[len_s-1-k]=tmp;
    }
    //printf("%s\n",s);
    while(s[0]){
        int a=0;//aの値をリセット
        for(i=0;i<4;i++){
            //printf("-------\n");
            int c=0,same=0;//カウントをリセット
            for(int j=0;t[i][j]!='\0';j++){
                //printf("t[%d][%d]=%c\n",i,j,t[i][j]);
                c++;
                if(s[j]==t[i][j]){
                    same++;
                }
            }
            //printf("c=%d,same=%d\n",c,same);
            if(c==same){//tの文字列の長さと同じだけ一致してたら
                a=same;//aにsameを保存
            }
            //
        }
        //printf("a=%d\n",a);
        if(a==0){
            printf("NO\n");
            return 0;
        }
        //4種と比べ終わったら
        for(int j=0;s[j]!='\0';j++){
            s[j]=s[j+a];
        }
    }
    printf("YES\n");
    return 0;
}

