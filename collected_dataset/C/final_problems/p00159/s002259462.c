// AOJ Volume 1 Problem 0159 The Best Body

#include <stdio.h>

#define ABS(x)  ((x) < 0.0 ? ((x) * -1.0) : (x))


double cal_bmi(int h, int w)
{
    double bmi;
    
    bmi = (double)w / ((h / 100.0) * (h / 100.0));
    
    return (bmi);
}

int main(void)
{
    int n;
    int i, h, w;
    double bmi;
    int ideal_no;
    double ideal_bmi;
    int j;

    while (1){
        scanf("%d", &n);
        if (n == 0){
            break;
        }
        scanf("%d%d%d", &i, &h, &w);

        ideal_no = i;
        ideal_bmi = cal_bmi(h, w);
        
        for (j = 1; j < n; j++){
            scanf("%d%d%d", &i, &h, &w);
            bmi = cal_bmi(h, w);
            if (ABS(22.0 - bmi) < ABS(22.0 - ideal_bmi)){
                ideal_no = i;
                ideal_bmi = bmi;
            }
        }
        printf("%d\n", ideal_no);
    }

    return (0);
}