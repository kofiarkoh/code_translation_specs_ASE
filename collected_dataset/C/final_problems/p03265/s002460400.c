/* ex 7_1
   TakayukiY*/

#include<stdio.h>

typedef struct point{
    int x_elem;
    int y_elem;
} point;

int main(void){
    //各頂点
    point p1,p2,p3,p4;
    //頂点1・2間の各成分の差分
    int x,y;
    //代入
    scanf("%d%d%d%d",&p1.x_elem,&p1.y_elem,&p2.x_elem,&p2.y_elem);
    //1と2の差分を計算
    x=p1.x_elem - p2.x_elem;
    y=p1.y_elem - p2.y_elem;
    //p3,p4を復元
    p3.x_elem=p2.x_elem+y;
    p3.y_elem=p2.y_elem-x;
    p4.x_elem=p1.x_elem+y;
    p4.y_elem=p1.y_elem-x;
    //出力
    printf("%d %d %d %d",p3.x_elem,p3.y_elem,p4.x_elem,p4.y_elem);

    return 0;
}