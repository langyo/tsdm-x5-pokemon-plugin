<?php

namespace pokemon;

use discuz_table;
use DB;

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

class table_pm_data extends discuz_table {

    public static function t() {
        static $_instance;
        if(!isset($_instance)) {
            $_instance = new self();
        }
        return $_instance;
    }

    public function __construct() {
        $this->_table = 'pm_data';
        $this->_pk = 'id';
        parent::__construct();
    }

    public function fetch_all() {
        return DB::fetch_all('SELECT * FROM %t ORDER BY id ASC', [$this->_table]);
    }

    public function fetch_by_mapid($mapid) {
        return DB::fetch_all('SELECT * FROM %t WHERE FIND_IN_SET(%d, mapid)', [$this->_table, $mapid]);
    }
}
