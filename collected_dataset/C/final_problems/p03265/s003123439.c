/* ex7_1
  kt125 */

#include<stdio.h>

//頂点を表す構造体
typedef struct point{
    int x_elem;
    int y_elem;
}point;

//p2を中心にp1を90°回転した点を返す関数
point shift_point(point p1, point p2){
    point p3;
    //p1からp2への方向ベクトルを反時計回り90°回転させて、p2に足してp3を求める
    p3.x_elem = p2.x_elem + (p2.y_elem - p1.y_elem);
    p3.y_elem = p2.y_elem + (p1.x_elem - p2.x_elem);
    return p3;
}

int main(void){
    point p1,p2,p3,p4; //各頂点
    scanf("%d %d %d %d", &p1.x_elem, &p1.y_elem, &p2.x_elem, &p2.y_elem);

    p4 = shift_point(p2, p1); //p1を中心にp2を90°回転
    p3 = shift_point(p1, p4); //p4を中心にp1を90°回転

    printf("%d %d %d %d\n", p3.x_elem, p3.y_elem, p4.x_elem, p4.y_elem);
    return 0;
}