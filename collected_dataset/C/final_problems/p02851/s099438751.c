#include <stdio.h>
#include <stdlib.h>

/* --- ring */

typedef struct ring {
    int val;
    struct ring *next, *prev;
} *ring_t;

ring_t ring_unshift (int val, ring_t ring) {
    ring_t cell = (ring_t)malloc(sizeof(struct ring));
    cell->val = val;

    if (!ring) {
        cell->next = cell;
        cell->prev = cell;
    } else {
        cell->next = ring;
        cell->prev = ring->prev;
        ring->prev = cell;
        cell->prev->next = cell;
    }

    return cell;
}

ring_t ring_pop (ring_t ring) {
    if (ring->next != ring) {
        ring->prev = ring->prev->prev;
        ring->prev->next = ring;
        return ring;
    } else {
        return NULL;
    }
}

/* --- skiplist */

typedef struct list {
    int id, count, level;
    ring_t ring;
    struct list* next[4];
} *list_t;

list_t res_buffer[4];

void list_search_prev_node (int id, list_t list) {
    int level = 3;

    if (!list) {
        res_buffer[3] = res_buffer[2] = res_buffer[1] = res_buffer[0] = NULL;
        return;
    }

    while (1) {
        if (list->next[level] && list->next[level]->id > id) list = list->next[level];
        else if (level) res_buffer[level--] = list;
        else { res_buffer[0] = list; break; };
    }
}

list_t list_add (int id, list_t list) {
    list_t cell = (list_t)calloc(1, sizeof(struct list));
    cell->id = id;

    if (!list) {
        cell->level = 3;
        return cell;
    }

    switch (rand() % 8) {
      case 0: case 1: case 2: case 3:
        cell->level = 0;
        break;
      case 4: case 5:
        cell->level = 1;
        break;
      case 6:
        cell->level = 2;
        break;
      default:
        cell->level = 3;
        break;
    }

    list_search_prev_node(id, list);
    for (int i = 0; i <= cell->level; i++) {
        cell->next[i] = res_buffer[i]->next[i];
        res_buffer[i]->next[i] = cell;
    }

    return list;
}

list_t list_find_cell (int id, list_t list) {
    list_search_prev_node(id, list);
    if (res_buffer[0]->next[0] && res_buffer[0]->next[0]->id == id) return res_buffer[0]->next[0];
    return NULL;
}

void unshift_to_cell (int index, list_t cell) {
    cell->count++;
    cell->ring = ring_unshift(index, cell->ring);
}

void pop_from_cell_le (int index, list_t cell) {
    while (cell->ring && cell->ring->prev->val <= index) {
        cell->count--;
        cell->ring = ring_pop(cell->ring);
    }
}

void test () {
    list_t list = NULL, cell;

    list = list_add(-1, list);  /* sentinel */
    list = list_add(20, list);

    cell = list_find_cell(20, list);
    unshift_to_cell(-1, cell);
    unshift_to_cell(5, cell);
    unshift_to_cell(6, cell);
    unshift_to_cell(7, cell);
    printf("%d == 4\n", cell->count);

    pop_from_cell_le(6, cell);
    printf("%d == 1\n", cell->count);

    pop_from_cell_le(100, cell);
    printf("%d == 0\n", cell->count);

    unshift_to_cell(5, cell);
    unshift_to_cell(6, cell);
    unshift_to_cell(7, cell);
    printf("%d == 3\n", cell->count);

    list = list_add(0, list);
    list = list_add(100, list);

    cell = list_find_cell(0, list);
    unshift_to_cell(5, cell);
    printf("%d == 1\n", cell->count);

    cell = list_find_cell(100, list);
    unshift_to_cell(5, cell);
    printf("%d == 1\n", cell->count);

    cell = list_find_cell(20, list);
    printf("%d == 3\n", cell->count);
}

/* --- */

int n, k, a[100001], count;

int main (void) {
    scanf("%d %d", &n, &k);
    for (int i = 0; i < n; i++) scanf("%d", &a[i]);

    /*  sum */
    a[0] = (a[0] - 1) % k;
    if (a[0] < 0) a[0] += k;
    for (int i = 1; i < n; i++){
        a[i] = (a[i - 1] + a[i] - 1) % k;
        if (a[i] < 0) a[i] += k;
    }

    /* solve */

    list_t list = list_add(0, list_add(-1, NULL));
    unshift_to_cell(-1, list_find_cell(0, list));

    for (int i = 0; i < n; i++) {
        list_t cell = list_find_cell(a[i], list);
        if (!cell) {
            list_add(a[i], list);
            cell = list_find_cell(a[i], list);
        }
        pop_from_cell_le(i - k, cell);
        if (cell->count) count += cell->count;
        unshift_to_cell(i, cell);
    }

    printf("%d\n", count);
}
