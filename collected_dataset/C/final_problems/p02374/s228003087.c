#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define DEBUG 0

int parent[100000];
int weight[100000];
int sum[100000];
int dirty[100000];
int clean[100000];

int Dirty;

void add(int v, int w);
int getSum(int u);

int main(){
    memset(parent,0,sizeof(int)*100000); //等同於new int[100000]
    memset(weight,0,sizeof(int)*100000);
    memset(sum,0,sizeof(int)*100000);
    memset(dirty,0,sizeof(int)*100000);
    memset(clean,0,sizeof(int)*100000);
   

    int n = 0;
    scanf("%d",&n);
    if(DEBUG)printf("n = %d\n",n);

    //初始化用的計數器變數
    int i = 0;                              // i = 這是第幾個點(同題目)
    int j = 0;                              // j = 這個點有幾個child

    int ki = 0;                             // ki = 題目中的ki 代表這個點有幾個child
    int ci = 0;                             // ci = 這個點i 的child 們 在此使用while 迴圈持續取值重複使用
    while(n > 0){
        //每當n 還沒歸零 代表還沒初始化到最後一個點
        
        scanf("%d",&ki);                    // scanf ki 來確定是否這個點有child
        while(ki > 0){                      // 當ki 大於零 代表這個點i 有ki 個child
            scanf("%d",&ci);                // scanf ci 取得誰是child
            parent[ci] = i;                 // 當ci 是點 i 的child 就把他的parent 設定成 i
            ki --;                          // 減少 ki 進入下一回圈
        }
        // 如果 ki == 0 或者 已經初始化完畢
        i ++;                               // i++來更新下一個點
        n --;                               // n--進入下一次迴圈
    }

    // 完成初始化階段，完成此階段後 對於任意點 i , parent[i] 代表他的parent 是誰
    // 在此題目中 每個點的parent 只會有一個 所以這個方法非常簡單暴力
    Dirty = 0;
    int q ;
    scanf("%d",&q);
    if(DEBUG)printf("q = %d\n",q);

    int cmd = 0;                            // cmd == 0 -> add  1 -> getSum
    int v = 0;                              // 題目中的v
    int w = 0;                              // 題目中的w
    int u = 0;                              // 題目中的u

    while(q>0){
        //每當q 還沒歸零 代表需要繼續更新重量或者查詢
        //我有強烈預感這要用lazy
        //或者要用dp
        scanf("%d",&cmd);                   // 取得cmd 確定要add or getSum
        if(DEBUG)printf("cmd = %d\n",cmd);

        if(cmd == 0){
            scanf("%d %d",&v,&w);
            add(v,w);
	    Dirty ++;
	    dirty[v] = 1;
	    memset(clean,0,sizeof(int)*100000);
        }else{
            scanf("%d",&u);
            printf("%d\n",getSum(u));
	    //dirty = 0;
        }
        q--;
    }

    return 0;
}


void add(int v,int w){                              //直接在weight 更新重量 題目意思是 由V到他parent 的這條路徑重量為w 但是首先 這條路徑唯一 (此題目每個項目的parent 只有一個) 且紀錄parent 我們用另一個矩陣紀錄了
    weight[v] += w;
    return;
}

int getSum(int u){
    if(u == 0){
	    return 0;
    }
    if(clean[u]){
	    return sum[u];
    }
    if(Dirty){
        while(u >0){
	    if(dirty[u]){dirty[u] = 0; Dirty--;}
	    clean[u] = 1;
	    sum[u] = weight[u] + getSum(parent[u]);
	    return sum[u];
	}
    }else{
	    return sum[u];
    }
}

