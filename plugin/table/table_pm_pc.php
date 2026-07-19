<?php

namespace pokemon;

use discuz_table;
use DB;

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

class table_pm_pc extends discuz_table {

    public static function t() {
        static $_instance;
        if(!isset($_instance)) {
            $_instance = new self();
        }
        return $_instance;
    }

    public function __construct() {
        $this->_table = 'pm_pc';
        $this->_pk = 'id';
        parent::__construct();
    }

    public function fetch_by_uid($uid) {
        return DB::fetch_all('SELECT * FROM %t WHERE uid=%d', [$this->_table, $uid]);
    }

    public function delete_by_uid_pmid($uid, $pm_id) {
        DB::query('DELETE FROM %t WHERE uid=%d AND pm_id=%d', [$this->_table, $uid, $pm_id]);
    }
}
