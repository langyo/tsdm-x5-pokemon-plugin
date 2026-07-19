<?php

namespace pokemon;

use discuz_table;
use DB;

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

class table_pm_itemdata extends discuz_table {

    public static function t() {
        static $_instance;
        if(!isset($_instance)) {
            $_instance = new self();
        }
        return $_instance;
    }

    public function __construct() {
        $this->_table = 'pm_itemdata';
        $this->_pk = 'id';
        parent::__construct();
    }

    public function fetch_all() {
        return DB::fetch_all('SELECT * FROM %t ORDER BY id ASC', [$this->_table]);
    }

    public function fetch_by_type($type) {
        return DB::fetch_all('SELECT * FROM %t WHERE type=%d', [$this->_table, $type]);
    }

    public function fetch_shop_items() {
        return DB::fetch_all('SELECT * FROM %t WHERE shop>0 ORDER BY id ASC', [$this->_table]);
    }
}
